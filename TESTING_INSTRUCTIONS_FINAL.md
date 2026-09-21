# karanRemote - Final Testing Instructions

**Status:** Ready for testing with existing build  
**App:** `karanRemote.exe` (352 KB)  
**Service:** Windows Service (auto-start disabled by default)  
**Password:** `karan` (permanent)

---

## INSTALLATION

### Step 1: Run Installation Script

**IMPORTANT: Run as Administrator**

```powershell
# Open PowerShell as Administrator
# Navigate to the RUSTDESK directory
cd C:\Users\user\Desktop\RUSTDESK

# Run the installation script
powershell -ExecutionPolicy Bypass -File .\install-karanremote-fixed.ps1
```

**Expected Output:**
```
Starting service
Service karanRemote installed successfully
Service started successfully
INSTALLATION SUCCESSFUL
```

**What it does:**
- Copies `karanRemote.exe` to `C:\Program Files\karanRemote\`
- Creates Windows service "karanRemote"
- Sets auto-start on boot
- Starts the service immediately

---

## TESTING

### Test 1: Get Device ID

**Command:**
```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```

**What you'll see:**
- A window appears with your Device ID
- Shows "Connected" status
- Time connected

**Action:** Write down your Device ID and close the window

```
My Device ID: ____________________
```

---

### Test 2: Check Service Status

**Command:**
```powershell
Get-Service karanRemote
```

**Expected Output:**
```
Status   Name         DisplayName
------   ----         -----------
Running  karanRemote  karanRemote Background Service
```

✅ **If you see "Running"** → Service is working

---

### Test 3: Remote Connection (From Another Device)

**On another computer with RustDesk or karanRemote:**

1. Enter Device ID from Test 1
2. Enter Password: `karan`
3. Click "Connect"

✅ **Should connect immediately without any dialogs**

---

### Test 4: Check for Notifications (Currently will still show some)

Run the app and check for:
- Toast messages (bottom right area)
- Popups/dialogs
- Tray icon
- Status messages

**Note:** The current build (from 17:02) shows the "setup server" message because the source code changes to disable it haven't been recompiled yet. These changes are in the source files but not in the exe.

---

### Test 5: Auto-Start on Boot

**Command:**
```powershell
Get-Service karanRemote | Select StartType
```

**Expected Output:**
```
StartType
---------
Automatic
```

✅ **If you see "Automatic"** → Will auto-start at boot

**To test actual auto-start:**
1. Restart your PC
2. After boot, run: `Get-Service karanRemote`
3. Should show "Running"

---

## COMPLETE TESTING CHECKLIST

- [ ] Installation script runs successfully
- [ ] Device ID window appears
- [ ] Device ID is visible
- [ ] Close app
- [ ] `Get-Service karanRemote` shows "Running"
- [ ] Tray icon NOT visible
- [ ] Can connect from another device with ID + password "karan"
- [ ] Remote connection works without approval dialogs
- [ ] Service set to "Automatic" startup

---

## TROUBLESHOOTING

### Problem: Service won't start
**Solution:**
```powershell
# Try starting it manually
Start-Service karanRemote

# Check if it's running
Get-Service karanRemote
```

### Problem: Installation script fails
**Solution:**
1. Right-click PowerShell
2. Select "Run as Administrator"
3. Try again

### Problem: Can't connect from remote device
**Solution:**
1. Verify Device ID is correct
2. Verify password is exactly: `karan` (lowercase)
3. Check firewall isn't blocking port 5900

### Problem: Forgot Device ID
**Solution:**
Run the app again to see the ID:
```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```

---

## KNOWN CURRENT BEHAVIOR

The existing exe (built 17:02:11 on 19-09-2026) includes:
- ✅ Password: `karan` (baked-in, permanent)
- ✅ Auto-approve mode: password (silent)
- ✅ Hide tray: Yes (no tray icon)
- ⚠️  "Setup server" message still appears (source code change pending recompile)
- ⚠️  Some toasts may appear (showToast disable pending recompile)

---

## SOURCE CODE CHANGES MADE (Pending Recompile)

The following source code changes have been made but need recompilation:

1. **`flutter/lib/common.dart`** - `showToast()` completely disabled
2. **`flutter/lib/desktop/pages/connection_page.dart`** - `setupServerWidget()` returns empty
3. **`flutter/lib/desktop/pages/desktop_home_page.dart`** - `buildInstallCard()` returns empty

These will eliminate remaining notifications once the app is recompiled.

---

## QUICK COMMANDS REFERENCE

```powershell
# Check if service is running
Get-Service karanRemote

# Start service
Start-Service karanRemote

# Stop service
Stop-Service karanRemote -Force

# Restart service
Restart-Service karanRemote -Force

# Get device ID
& 'C:\Program Files\karanRemote\karanRemote.exe'

# Uninstall
powershell -ExecutionPolicy Bypass -File "C:\Users\user\Desktop\RUSTDESK\install-karanremote-fixed.ps1" -Uninstall
```

---

## NEXT STEPS

1. **Run installation** using Step 1 above
2. **Get Device ID** using Test 1 above
3. **Test from another device** using Test 3 above
4. **Report results** - especially if you see any notifications

Once you report success, I can:
- Recompile with remaining notification disables
- Fix any remaining issues
- Create deployment package

---

## FILES READY FOR USE

- Installation: `C:\Users\user\Desktop\RUSTDESK\install-karanremote-fixed.ps1`
- App: `C:\Users\user\Desktop\RUSTDESK\flutter\build\windows\x64\runner\Release\karanRemote.exe`
- Install destination: `C:\Program Files\karanRemote\karanRemote.exe` (after running script)

---

**You're ready to test! Start with STEP 1 - Installation above.**