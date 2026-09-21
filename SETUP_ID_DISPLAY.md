# karanRemote - Show ID on First Launch, Then Silent

## Behavior

This customized version of karanRemote is designed to:

1. **First Launch**: Show the connection ID window so you can write down or copy the ID
2. **After Close**: Run silently in the background with no UI, no tray, no notifications
3. **On Restart**: Auto-start as a silent background service
4. **Connections**: Accept remote connections with password "karan" automatically

## Installation

### Step 1: Install the Service
```powershell
# Run as Administrator:
powershell -ExecutionPolicy Bypass -File .\install-karanremote.ps1
```

### Step 2: First Launch to Get ID
After installation, run the application once manually:
```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```

This will:
- Display a window showing your device ID (e.g., 123456789)
- Show the connection status
- **Write down or take a screenshot of this ID** - you'll need it to connect

### Step 3: Close the Window
Simply close the window - the service will continue running in the background silently.

## How It Works

### Window Behavior
1. **Manual Run**: Shows full UI with ID window
2. **Service Mode** (`--service` flag): Runs silently, no UI, background only
3. **Password**: "karan" (permanent, baked into build)
4. **Connections**: Automatic with correct password (no approval needed)

### Auto-Start
- Service restarts automatically if stopped
- Auto-starts at PC boot
- Continues running even if you close any windows

## Connection Details

### To Connect from Another Device

1. **Get the ID** (from the first launch window)
2. **In another karanRemote/RustDesk app**:
   - Enter the ID: (the number from step 1)
   - Enter password: `karan`
   - Click Connect
   - Access granted immediately (no approval needed)

### Direct Connection String
You can also connect directly using:
```
karanRemote://[ID]:[PASSWORD]@[THIS_PC]
```

## Uninstall

To uninstall the service completely:
```powershell
# Run as Administrator:
powershell -ExecutionPolicy Bypass -File .\install-karanremote.ps1 -Uninstall
```

This will:
- Stop the Windows service
- Remove from Program Files
- Remove auto-start registry entries
- Clean up startup shortcuts

## Troubleshooting

### I need to see the ID again
Run this command manually to show the window once:
```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```

### Service won't start
Check service status:
```powershell
Get-Service karanRemote
```

### Can't connect with password
- Verify you're using password: `karan` (lowercase)
- Ensure the service is running: `Get-Service karanRemote`
- Check Windows Firewall might be blocking connections

## Technical Details

- **App Name**: karanRemote
- **Password**: karan (SHA256 hashed + salted, cannot be changed)
- **Service**: Windows Service (runs as SYSTEM)
- **Auto-start**: Yes (boot startup)
- **UI Mode**: Silent when run with `--service` flag
- **Connections**: Silent (approve-mode=password)
- **No Notifications**: Configured to be completely silent