# karanRemote - Quick Start Guide

## What You Have

A customized version of RustDesk that:
- Shows your connection ID on first launch
- Runs silently in background afterwards (no UI, no tray, no notifications)
- Accepts connections automatically with password `karan`
- Auto-starts at every PC boot
- No approval dialogs or popups

## Installation & First Launch

### 1. Install as Service (Run as Administrator)

```powershell
powershell -ExecutionPolicy Bypass -File .\install-karanremote.ps1
```

This will:
- Copy files to `C:\Program Files\karanRemote\`
- Create a Windows service
- Start the service immediately
- Configure auto-start at boot

### 2. Get Your Device ID (Important!)

Run the app once manually to see your ID:

```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```

A window will appear showing:
- Your **Device ID** (e.g., `123456789`)
- Your connection status
- Other device info

**→ Write down your Device ID!**

### 3. Close the Window

Simply close the window. The service will:
- Continue running in the background
- Accept connections with password `karan`
- Make no sound or show no notifications
- Auto-restart if it ever stops

## Connecting from Another Device

### Using Another karanRemote/RustDesk:

1. Open karanRemote on another computer
2. Click "Connect"
3. Enter:
   - **Device ID**: The number you wrote down (e.g., `123456789`)
   - **Password**: `karan`
4. Click "Connect" → Access granted immediately!

### Connection Details:

- **Password**: `karan` (cannot be changed)
- **Approval**: Not needed (automatic with correct password)
- **Notifications**: None (silent)
- **Keep running**: Yes, even after you close windows

## After Reboot

The service will automatically:
- Start when Windows boots
- Run silently in background
- Accept connections with password `karan`
- Require no interaction from you

## If You Need the ID Again

Just run the same command:
```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```

The window will show your ID again.

## Uninstall

To completely remove it:

```powershell
powershell -ExecutionPolicy Bypass -File .\install-karanremote.ps1 -Uninstall
```

This will:
- Stop the service
- Remove from Program Files
- Clean up auto-start configuration
- Remove all shortcuts

## Troubleshooting

### Service won't start
```powershell
Get-Service karanRemote
```
Check if it shows "Running" status.

### Forgot the device ID
```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```
This will show it again.

### Can't connect from remote device
- Check password is `karan` (lowercase)
- Verify service is running: `Get-Service karanRemote`
- Windows Firewall might block it - check firewall rules
- Both devices need network connectivity

### Want to change settings
The app is locked to:
- Device ID: Cannot be changed (unique)
- Password: `karan` (permanent, baked-in)
- All settings: Fixed to ensure silent operation

## Key Features

✓ Shows ID on first run  
✓ Runs silently afterwards  
✓ Auto-starts at boot  
✓ No UI/notifications  
✓ Automatic connections with password  
✓ Works even after closing all windows  
✓ Restarts if it crashes  
✓ Cannot be easily uninstalled (by design)  

## File Locations

- **Install**: `C:\Program Files\karanRemote\`
- **Config**: `%APPDATA%\karanRemote\`
- **Service Name**: `karanRemote`
- **Executable**: `karanRemote.exe`