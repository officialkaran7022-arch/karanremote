use std::{
    collections::HashMap,
    iter::FromIterator,
    sync::{Arc, Mutex},
};

use sciter::Value;

use hbb_common::{
    allow_err,
    config::{LocalConfig, PeerConfig},
    log,
};

#[cfg(not(feature = "flutter"))]
use crate::ui_session_interface::Session;
use crate::{common::get_app_name, ipc, ui_interface::*};

mod cm;
#[cfg(feature = "inline")]
pub mod inline;
pub mod remote;

#[allow(dead_code)]
type Status = (i32, bool, i64, String);

lazy_static::lazy_static! {
    // stupid workaround for https://sciter.com/forums/topic/crash-on-latest-tis-mac-sdk-sometimes/
    static ref STUPID_VALUES: Mutex<Vec<Arc<Vec<Value>>>> = Default::default();
}

#[cfg(not(feature = "flutter"))]
lazy_static::lazy_static! {
    pub static ref CUR_SESSION: Arc<Mutex<Option<Session<remote::SciterHandler>>>> = Default::default();
}

struct UIHostHandler;

/// UI resources (index.html / cm.html / ...) live in `src/ui` next to the
/// executable. The working directory is not reliable here: the connection
/// manager sub-process is spawned by the background server (System32 cwd),
/// so prefer the executable's directory and only fall back to the cwd.
#[cfg(not(feature = "inline"))]
fn ui_base_dir() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let cand = parent.join("src").join("ui");
            log::debug!(
                "ui_base_dir: exe={:?} cand={:?} index_exists={}",
                exe,
                cand,
                cand.join("index.html").exists()
            );
            if cand.join("index.html").exists() {
                return parent.display().to_string();
            }
        }
    }
    std::env::current_dir()
        .map(|c| c.display().to_string())
        .unwrap_or_default()
}

pub fn start(args: &mut [String]) {
    #[cfg(target_os = "macos")]
    crate::platform::delegate::show_dock();
    #[cfg(all(target_os = "linux", feature = "inline"))]
    {
        let app_dir = std::env::var("APPDIR").unwrap_or("".to_string());
        let mut so_path = "/usr/share/rustdesk/libsciter-gtk.so".to_owned();
        for (prefix, dir) in [
            ("", "/usr"),
            ("", "/app"),
            (&app_dir, "/usr"),
            (&app_dir, "/app"),
        ]
        .iter()
        {
            let path = format!("{prefix}{dir}/share/rustdesk/libsciter-gtk.so");
            if std::path::Path::new(&path).exists() {
                so_path = path;
                break;
            }
        }
        sciter::set_library(&so_path).ok();
    }
    #[cfg(all(windows, feature = "inline"))]
    {
        use std::io::Write;
        let dir = std::env::temp_dir().join("rustdesk");
        let path = dir.join("sciter.dll");
        if std::fs::create_dir_all(&dir).is_ok() && !path.exists() {
            let mut f = std::fs::File::create(&path);
            if let Ok(f) = &mut f {
                f.write_all(include_bytes!("../sciter.dll")).ok();
            }
        }
        let p = path.to_string_lossy().to_string();
        log::debug!("Loaded embedded sciter: {}", p);
        sciter::set_library(&p).ok();
    }
    #[cfg(windows)]
    // Check if there is a sciter.dll nearby.
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let sciter_dll_path = parent.join("sciter.dll");
            if sciter_dll_path.exists() {
                // Try to set the sciter dll.
                let p = sciter_dll_path.to_string_lossy().to_string();
                log::debug!("Found dll:{}, \n {:?}", p, sciter::set_library(&p));
            }
        }
    }
    // https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-types.h
    // https://github.com/rustdesk/rustdesk/issues/132#issuecomment-886069737
    #[cfg(windows)]
    allow_err!(sciter::set_options(sciter::RuntimeOptions::GfxLayer(
        sciter::GFX_LAYER::WARP
    )));
    use sciter::SCRIPT_RUNTIME_FEATURES::*;
    allow_err!(sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        ALLOW_FILE_IO as u8 | ALLOW_SOCKET_IO as u8 | ALLOW_EVAL as u8 | ALLOW_SYSINFO as u8
    )));
    let mut frame = sciter::WindowBuilder::main_window().create();
    #[cfg(windows)]
    allow_err!(sciter::set_options(sciter::RuntimeOptions::UxTheming(true)));
    frame.set_title(&crate::get_app_name());
    #[cfg(target_os = "macos")]
    crate::platform::delegate::make_menubar(frame.get_host(), args.is_empty());
    #[cfg(windows)]
    crate::platform::try_set_window_foreground(frame.get_hwnd() as _);
    let page;
    if args.len() > 1 && args[0] == "--play" {
        args[0] = "--connect".to_owned();
        let path: std::path::PathBuf = (&args[1]).into();
        let id = path
            .file_stem()
            .map(|p| p.to_str().unwrap_or(""))
            .unwrap_or("")
            .to_owned();
        args[1] = id;
    }
    if args.is_empty() {
        std::thread::spawn(move || check_zombie());
        crate::common::check_software_update();
        frame.event_handler(UI {});
        frame.sciter_handler(UIHostHandler {});
        page = "index.html";
        // Start pulse audio local server.
        #[cfg(target_os = "linux")]
        std::thread::spawn(crate::ipc::start_pa);
    } else if args[0] == "--install" {
        frame.event_handler(UI {});
        frame.sciter_handler(UIHostHandler {});
        page = "install.html";
    } else if args[0] == "--cm" {
        frame.register_behavior("connection-manager", move || {
            Box::new(cm::SciterConnectionManager::new())
        });
        page = "cm.html";
        *cm::HIDE_CM.lock().unwrap() = crate::ipc::get_config("hide_cm")
            .ok()
            .flatten()
            .unwrap_or_default()
            == "true";
    } else if (args[0] == "--connect"
        || args[0] == "--file-transfer"
        || args[0] == "--port-forward"
        || args[0] == "--rdp")
        && args.len() > 1
    {
        #[cfg(windows)]
        {
            let hw = frame.get_host().get_hwnd();
            crate::platform::windows::enable_lowlevel_keyboard(hw as _);
        }
        let mut iter = args.iter();
        let Some(cmd) = iter.next() else {
            log::error!("Failed to get cmd arg");
            return;
        };
        let cmd = cmd.to_owned();
        let Some(id) = iter.next() else {
            log::error!("Failed to get id arg");
            return;
        };
        let id = id.to_owned();
        let pass = iter.next().unwrap_or(&"".to_owned()).clone();
        let args: Vec<String> = iter.map(|x| x.clone()).collect();
        frame.set_title(&id);
        frame.register_behavior("native-remote", move || {
            let handler =
                remote::SciterSession::new(cmd.clone(), id.clone(), pass.clone(), args.clone());
            #[cfg(not(feature = "flutter"))]
            {
                *CUR_SESSION.lock().unwrap() = Some(handler.inner());
            }
            Box::new(handler)
        });
        page = "remote.html";
    } else {
        log::error!("Wrong command: {:?}", args);
        return;
    }
    #[cfg(feature = "inline")]
    {
        let html = if page == "index.html" {
            inline::get_index()
        } else if page == "cm.html" {
            inline::get_cm()
        } else if page == "install.html" {
            inline::get_install()
        } else {
            inline::get_remote()
        };
        frame.load_html(html.as_bytes(), Some(page));
    }
    #[cfg(not(feature = "inline"))]
    frame.load_file(&format!("file://{}/src/ui/{}", ui_base_dir(), page));
    let hide_cm = *cm::HIDE_CM.lock().unwrap();
    if !args.is_empty() && args[0] == "--cm" && hide_cm {
        // run_app calls expand(show) + run_loop, we use collapse(hide) + run_loop instead to create a hidden window
        frame.collapse(true);
        frame.run_loop();
        return;
    }
    frame.run_app();
}

struct UI {}

impl UI {
    fn recent_sessions_updated(&self) -> bool {
        recent_sessions_updated()
    }

    fn get_id(&self) -> String {
        ipc::get_id()
    }

    fn temporary_password(&mut self) -> String {
        temporary_password()
    }

    fn update_temporary_password(&self) {
        update_temporary_password()
    }

    fn set_permanent_password(&self, password: String) {
        let _ = set_permanent_password_with_result(password);
    }

    fn is_local_permanent_password_set(&self) -> bool {
        is_local_permanent_password_set()
    }

    fn is_permanent_password_set(&self) -> bool {
        is_permanent_password_set()
    }

    fn get_remote_id(&mut self) -> String {
        LocalConfig::get_remote_id()
    }

    fn set_remote_id(&mut self, id: String) {
        LocalConfig::set_remote_id(&id);
    }

    fn goto_install(&mut self) {
        goto_install();
    }

    fn install_me(&mut self, _options: String, _path: String) {
        install_me(_options, _path, false, false);
    }

    fn update_me(&self, _path: String) {
        update_me(_path);
    }

    fn run_without_install(&self) {
        run_without_install();
    }

    fn show_run_without_install(&self) -> bool {
        show_run_without_install()
    }

    fn get_license(&self) -> String {
        get_license()
    }

    fn get_option(&self, key: String) -> String {
        get_option(key)
    }

    fn get_local_option(&self, key: String) -> String {
        get_local_option(key)
    }

    fn set_local_option(&self, key: String, value: String) {
        set_local_option(key, value);
    }

    fn peer_has_password(&self, id: String) -> bool {
        peer_has_password(id)
    }

    fn forget_password(&self, id: String) {
        forget_password(id)
    }

    fn get_peer_option(&self, id: String, name: String) -> String {
        get_peer_option(id, name)
    }

    fn set_peer_option(&self, id: String, name: String, value: String) {
        set_peer_option(id, name, value)
    }

    fn using_public_server(&self) -> bool {
        crate::using_public_server()
    }

    fn is_incoming_only(&self) -> bool {
        hbb_common::config::is_incoming_only()
    }

    pub fn is_outgoing_only(&self) -> bool {
        hbb_common::config::is_outgoing_only()
    }

    pub fn is_custom_client(&self) -> bool {
        crate::common::is_custom_client()
    }

    pub fn is_disable_settings(&self) -> bool {
        hbb_common::config::is_disable_settings()
    }

    pub fn is_disable_account(&self) -> bool {
        hbb_common::config::is_disable_account()
    }

    pub fn is_disable_installation(&self) -> bool {
        hbb_common::config::is_disable_installation()
    }

    pub fn is_disable_ab(&self) -> bool {
        hbb_common::config::is_disable_ab()
    }

    fn get_options(&self) -> Value {
        let hashmap: HashMap<String, String> =
            serde_json::from_str(&get_options()).unwrap_or_default();
        let mut m = Value::map();
        for (k, v) in hashmap {
            m.set_item(k, v);
        }
        m
    }

    fn test_if_valid_server(&self, host: String, test_with_proxy: bool) -> String {
        test_if_valid_server(host, test_with_proxy)
    }

    fn get_sound_inputs(&self) -> Value {
        Value::from_iter(get_sound_inputs())
    }

    fn set_options(&self, v: Value) {
        let mut m = HashMap::new();
        for (k, v) in v.items() {
            if let Some(k) = k.as_string() {
                if let Some(v) = v.as_string() {
                    if !v.is_empty() {
                        m.insert(k, v);
                    }
                }
            }
        }
        set_options(m);
    }

    fn set_option(&self, key: String, value: String) {
        set_option(key, value);
    }

    fn install_path(&mut self) -> String {
        install_path()
    }

    fn install_options(&self) -> String {
        install_options()
    }

    fn get_socks(&self) -> Value {
        Value::from_iter(get_socks())
    }

    fn set_socks(&self, proxy: String, username: String, password: String) {
        set_socks(proxy, username, password)
    }

    fn is_installed(&self) -> bool {
        is_installed()
    }

    fn get_supported_privacy_mode_impls(&self) -> String {
        serde_json::to_string(&crate::privacy_mode::get_supported_privacy_mode_impl())
            .unwrap_or_default()
    }

    fn is_root(&self) -> bool {
        is_root()
    }

    fn is_release(&self) -> bool {
        #[cfg(not(debug_assertions))]
        return true;
        #[cfg(debug_assertions)]
        return false;
    }

    fn is_share_rdp(&self) -> bool {
        is_share_rdp()
    }

    fn set_share_rdp(&self, _enable: bool) {
        set_share_rdp(_enable);
    }

    fn is_installed_lower_version(&self) -> bool {
        is_installed_lower_version()
    }

    fn closing(&mut self, x: i32, y: i32, w: i32, h: i32) {
        crate::server::input_service::fix_key_down_timeout_at_exit();
        LocalConfig::set_size(x, y, w, h);
    }

    fn get_size(&mut self) -> Value {
        let s = LocalConfig::get_size();
        let mut v = Vec::new();
        v.push(s.0);
        v.push(s.1);
        v.push(s.2);
        v.push(s.3);
        Value::from_iter(v)
    }

    fn get_mouse_time(&self) -> f64 {
        get_mouse_time()
    }

    fn check_mouse_time(&self) {
        check_mouse_time()
    }

    fn get_connect_status(&mut self) -> Value {
        let mut v = Value::array(0);
        let x = get_connect_status();
        v.push(x.status_num);
        v.push(x.key_confirmed);
        v.push(x.id);
        v
    }

    #[inline]
    fn get_peer_value(id: String, p: PeerConfig) -> Value {
        let values = vec![
            id,
            p.info.username.clone(),
            p.info.hostname.clone(),
            p.info.platform.clone(),
            p.options.get("alias").unwrap_or(&"".to_owned()).to_owned(),
        ];
        Value::from_iter(values)
    }

    fn get_peer(&self, id: String) -> Value {
        let c = get_peer(id.clone());
        Self::get_peer_value(id, c)
    }

    fn get_fav(&self) -> Value {
        Value::from_iter(get_fav())
    }

    fn store_fav(&self, fav: Value) {
        let mut tmp = vec![];
        fav.values().for_each(|v| {
            if let Some(v) = v.as_string() {
                if !v.is_empty() {
                    tmp.push(v);
                }
            }
        });
        store_fav(tmp);
    }

    fn get_recent_sessions(&mut self) -> Value {
        // to-do: limit number of recent sessions, and remove old peer file
        let peers: Vec<Value> = PeerConfig::peers(None)
            .drain(..)
            .map(|p| Self::get_peer_value(p.0, p.2))
            .collect();
        Value::from_iter(peers)
    }

    fn get_icon(&mut self) -> String {
        get_icon()
    }

    fn remove_peer(&mut self, id: String) {
        PeerConfig::remove(&id);
    }

    fn remove_discovered(&mut self, id: String) {
        remove_discovered(id);
    }

    fn send_wol(&mut self, id: String) {
        crate::lan::send_wol(id)
    }

    fn new_remote(&mut self, id: String, remote_type: String, force_relay: bool) {
        new_remote(id, remote_type, force_relay)
    }

    fn is_process_trusted(&mut self, _prompt: bool) -> bool {
        is_process_trusted(_prompt)
    }

    fn is_can_screen_recording(&mut self, _prompt: bool) -> bool {
        is_can_screen_recording(_prompt)
    }

    fn is_installed_daemon(&mut self, _prompt: bool) -> bool {
        is_installed_daemon(_prompt)
    }

    fn get_error(&mut self) -> String {
        get_error()
    }

    fn is_login_wayland(&mut self) -> bool {
        is_login_wayland()
    }

    fn current_is_wayland(&mut self) -> bool {
        current_is_wayland()
    }

    fn get_software_update_url(&self) -> String {
        crate::SOFTWARE_UPDATE_URL.lock().unwrap().clone()
    }

    fn get_new_version(&self) -> String {
        get_new_version()
    }

    fn get_version(&self) -> String {
        get_version()
    }

    fn get_fingerprint(&self) -> String {
        get_fingerprint()
    }

    fn get_app_name(&self) -> String {
        get_app_name()
    }

    fn get_software_ext(&self) -> String {
        #[cfg(windows)]
        let p = "exe";
        #[cfg(target_os = "macos")]
        let p = "dmg";
        #[cfg(target_os = "linux")]
        let p = "deb";
        p.to_owned()
    }

    fn get_software_store_path(&self) -> String {
        let mut p = std::env::temp_dir();
        let name = crate::SOFTWARE_UPDATE_URL
            .lock()
            .unwrap()
            .split("/")
            .last()
            .map(|x| x.to_owned())
            .unwrap_or(crate::get_app_name());
        p.push(name);
        format!("{}.{}", p.to_string_lossy(), self.get_software_ext())
    }

    fn create_shortcut(&self, _id: String) {
        #[cfg(windows)]
        create_shortcut(_id)
    }

    fn discover(&self) {
        std::thread::spawn(move || {
            allow_err!(crate::lan::discover());
        });
    }

    fn get_lan_peers(&self) -> String {
        // let peers = get_lan_peers()
        //     .into_iter()
        //     .map(|mut peer| {
        //         (
        //             peer.remove("id").unwrap_or_default(),
        //             peer.remove("username").unwrap_or_default(),
        //             peer.remove("hostname").unwrap_or_default(),
        //             peer.remove("platform").unwrap_or_default(),
        //         )
        //     })
        //     .collect::<Vec<(String, String, String, String)>>();
        serde_json::to_string(&get_lan_peers()).unwrap_or_default()
    }

    fn get_uuid(&self) -> String {
        get_uuid()
    }

    fn open_url(&self, url: String) {
        #[cfg(windows)]
        let p = "explorer";
        #[cfg(target_os = "macos")]
        let p = "open";
        #[cfg(target_os = "linux")]
        let p = if std::path::Path::new("/usr/bin/firefox").exists() {
            "firefox"
        } else {
            "xdg-open"
        };
        allow_err!(std::process::Command::new(p).arg(url).spawn());
    }

    fn change_id(&self, id: String) {
        reset_async_job_status();
        let old_id = self.get_id();
        change_id_shared(id, old_id);
    }

    fn http_request(&self, url: String, method: String, body: Option<String>, header: String) {
        http_request(url, method, body, header)
    }

    fn post_request(&self, url: String, body: String, header: String) {
        post_request(url, body, header)
    }

    fn is_ok_change_id(&self) -> bool {
        hbb_common::machine_uid::get().is_ok()
    }

    fn get_async_job_status(&self) -> String {
        get_async_job_status()
    }

    fn get_http_status(&self, url: String) -> Option<String> {
        get_async_http_status(url)
    }

    fn t(&self, name: String) -> String {
        crate::client::translate(name)
    }

    fn is_xfce(&self) -> bool {
        crate::platform::is_xfce()
    }

    fn get_api_server(&self) -> String {
        get_api_server()
    }

    fn has_hwcodec(&self) -> bool {
        has_hwcodec()
    }

    fn has_vram(&self) -> bool {
        has_vram()
    }

    fn get_langs(&self) -> String {
        get_langs()
    }

    fn video_save_directory(&self, root: bool) -> String {
        video_save_directory(root)
    }

    fn handle_relay_id(&self, id: String) -> String {
        handle_relay_id(&id).to_owned()
    }

    fn get_login_device_info(&self) -> String {
        get_login_device_info_json()
    }

    fn support_remove_wallpaper(&self) -> bool {
        support_remove_wallpaper()
    }

    fn has_valid_2fa(&self) -> bool {
        has_valid_2fa()
    }

    fn generate2fa(&self) -> String {
        generate2fa()
    }

    pub fn verify2fa(&self, code: String) -> bool {
        verify2fa(code)
    }

    fn verify_login(&self, raw: String, id: String) -> bool {
        crate::verify_login(&raw, &id)
    }

    fn generate_2fa_img_src(&self, data: String) -> String {
        let v = qrcode_generator::to_png_to_vec(data, qrcode_generator::QrCodeEcc::Low, 128)
            .unwrap_or_default();
        let s = hbb_common::sodiumoxide::base64::encode(
            v,
            hbb_common::sodiumoxide::base64::Variant::Original,
        );
        format!("data:image/png;base64,{s}")
    }

    pub fn check_hwcodec(&self) {
        check_hwcodec()
    }

    fn is_option_fixed(&self, key: String) -> bool {
        crate::ui_interface::is_option_fixed(&key)
    }

    fn get_builtin_option(&self, key: String) -> String {
        crate::ui_interface::get_builtin_option(&key)
    }

    fn is_remote_modify_enabled_by_control_permissions(&self) -> String {
        match crate::ui_interface::is_remote_modify_enabled_by_control_permissions() {
            Some(true) => "true",
            Some(false) => "false",
            None => "",
        }
        .to_string()
    }
}

impl sciter::EventHandler for UI {
    sciter::dispatch_script_call! {
        fn t(String);
        fn get_api_server();
        fn is_xfce();
        fn using_public_server();
        fn is_custom_client();
        fn is_outgoing_only();
        fn is_incoming_only();
        fn is_disable_settings();
        fn is_disable_account();
        fn is_disable_installation();
        fn is_disable_ab();
        fn get_id();
        fn temporary_password();
        fn update_temporary_password();
        fn set_permanent_password(String);
        fn is_local_permanent_password_set();
        fn is_permanent_password_set();
        fn get_remote_id();
        fn set_remote_id(String);
        fn closing(i32, i32, i32, i32);
        fn get_size();
        fn new_remote(String, String, bool);
        fn send_wol(String);
        fn remove_peer(String);
        fn remove_discovered(String);
        fn get_connect_status();
        fn get_mouse_time();
        fn check_mouse_time();
        fn get_recent_sessions();
        fn get_peer(String);
        fn get_fav();
        fn store_fav(Value);
        fn recent_sessions_updated();
        fn get_icon();
        fn install_me(String, String);
        fn is_installed();
        fn get_supported_privacy_mode_impls();
        fn is_root();
        fn is_release();
        fn set_socks(String, String, String);
        fn get_socks();
        fn is_share_rdp();
        fn set_share_rdp(bool);
        fn is_installed_lower_version();
        fn install_path();
        fn install_options();
        fn goto_install();
        fn is_process_trusted(bool);
        fn is_can_screen_recording(bool);
        fn is_installed_daemon(bool);
        fn get_error();
        fn is_login_wayland();
        fn current_is_wayland();
        fn get_options();
        fn get_option(String);
        fn get_local_option(String);
        fn set_local_option(String, String);
        fn get_peer_option(String, String);
        fn peer_has_password(String);
        fn forget_password(String);
        fn set_peer_option(String, String, String);
        fn get_license();
        fn test_if_valid_server(String, bool);
        fn get_sound_inputs();
        fn set_options(Value);
        fn set_option(String, String);
        fn get_software_update_url();
        fn get_new_version();
        fn get_version();
        fn get_fingerprint();
        fn update_me(String);
        fn show_run_without_install();
        fn run_without_install();
        fn get_app_name();
        fn get_software_store_path();
        fn get_software_ext();
        fn open_url(String);
        fn change_id(String);
        fn get_async_job_status();
        fn post_request(String, String, String);
        fn is_ok_change_id();
        fn create_shortcut(String);
        fn discover();
        fn get_lan_peers();
        fn get_uuid();
        fn has_hwcodec();
        fn has_vram();
        fn get_langs();
        fn video_save_directory(bool);
        fn handle_relay_id(String);
        fn get_login_device_info();
        fn support_remove_wallpaper();
        fn has_valid_2fa();
        fn generate2fa();
        fn generate_2fa_img_src(String);
        fn verify2fa(String);
        fn check_hwcodec();
        fn verify_login(String, String);
        fn is_option_fixed(String);
        fn get_builtin_option(String);
        fn is_remote_modify_enabled_by_control_permissions();
    }
}

impl sciter::host::HostHandler for UIHostHandler {
    fn on_graphics_critical_failure(&mut self) {
        log::error!("Critical rendering error: e.g. DirectX gfx driver error. Most probably bad gfx drivers.");
    }
}

#[cfg(not(target_os = "linux"))]
fn get_sound_inputs() -> Vec<String> {
    let mut out = Vec::new();
    use cpal::traits::{DeviceTrait, HostTrait};
    let host = cpal::default_host();
    if let Ok(devices) = host.devices() {
        for device in devices {
            if device.default_input_config().is_err() {
                continue;
            }
            if let Ok(name) = device.name() {
                out.push(name);
            }
        }
    }
    out
}

#[cfg(target_os = "linux")]
fn get_sound_inputs() -> Vec<String> {
    crate::platform::linux::get_pa_sources()
        .drain(..)
        .map(|x| x.1)
        .collect()
}

// sacrifice some memory
pub fn value_crash_workaround(values: &[Value]) -> Arc<Vec<Value>> {
    let persist = Arc::new(values.to_vec());
    STUPID_VALUES.lock().unwrap().push(persist.clone());
    persist
}

pub fn get_icon() -> String {
    // 128x128
    #[cfg(target_os = "macos")]
    // 128x128 on 160x160 canvas, then shrink to 128, mac looks better with padding
    {
        "data:image/png;base64,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".into()
    }
    #[cfg(not(target_os = "macos"))] // 128x128 no padding
    {
        "data:image/png;base64,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".into()
    }
}
