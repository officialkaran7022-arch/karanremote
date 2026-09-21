# karanRemote - How to Test and Run (Step by Step)

## QUICK START (3 Simple Steps)

### STEP 1: INSTALL SERVICE (Administrator)

**Option A: Click to Install (Easiest)**
1. Open File Explorer
2. Navigate to: `C:\Users\user\Desktop\RUSTDESK\`
3. Right-click on: `install-karanremote-fixed.ps1`
4. Select: "Run with PowerShell"
5. Click "Yes" when asked "Do you want to run this file?"
6. Wait for message: **"INSTALLATION SUCCESSFUL"**

**Option B: Command Line (If Option A doesn't work)**
1. Press: `Windows Key + R`
2. Type: `powershell`
3. Right-click PowerShell → "Run as Administrator"
4. Copy-paste this command:
   ```powershell
   cd C:\Users\user\Desktop\RUSTDESK
   powershell -ExecutionPolicy Bypass -File .\install-karanremote-fixed.ps1
   ```
5. Press Enter
6. Wait for: **"INSTALLATION SUCCESSFUL"** message

**What Happens:**
- ✅ Service installed to: `C:\Program Files\karanRemote\`
- ✅ Windows service "karanRemote" created
- ✅ Auto-start at boot configured
- ✅ Service started immediately

---

### STEP 2: GET YOUR DEVICE ID (First Time Only)

**Run this command in PowerShell (NOT as admin):**

```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```

**What You'll See:**
- A window appears with a blue card showing:
  - Device ID: `1244378915` (your number will be different)
  - "Connected" status
  - Time connected

**IMPORTANT: Write Down Your Device ID!**
```
Example: 1244378915
Your ID: _________________ (write it here)
```

**After Seeing ID:**
- Just close the window (click the X button)
- Service continues running in background
- **NO notifications will appear**

---

### STEP 3: TEST FROM ANOTHER DEVICE

**On another computer (or another karanRemote instance):**

1. Open karanRemote or RustDesk app
2. Enter the Device ID you wrote down
   - Example: `1244378915`
3. Enter Password: `karan`
4. Click "Connect"
5. **Instant access - no approval dialog needed!**

---

## DETAILED TESTING GUIDE

### TEST 1: Verify Service Installed

**Run this in PowerShell (regular user):**

```powershell
Get-Service karanRemote
```

**Expected Output:**
```
Status   Name         DisplayName
Running  karanRemote  karanRemote Background Service
```

**If you see "Running" - ✅ Installation successful!**

---

### TEST 2: Verify Auto-Start Configured

**Run this in PowerShell:**

```powershell
Get-Service karanRemote | Select StartType
```

**Expected Output:**
```
StartType
---------
Automatic
```

**If you see "Automatic" - ✅ Auto-start is configured!**

---

### TEST 3: Verify No Tray Icon

**Check:**
1. Look at taskbar bottom right
2. Look at system tray (click the ^ arrow if present)
3. **You should NOT see any karanRemote icon**

**If NO icon visible - ✅ Silent mode working!**

---

### TEST 4: Verify No Notifications

**After installation, check for:**
- ❌ Toast messages
- ❌ Popups
- ❌ Notifications
- ❌ Status messages
- ❌ Any visible UI

**If nothing appears - ✅ No notifications working!**

---

### TEST 5: Test Remote Connection

**Setup:**
- Device A: Your PC with karanRemote installed
- Device B: Another computer with karanRemote/RustDesk

**On Device B:**
1. Open karanRemote or RustDesk
2. Enter Device ID from Step 2
3. Enter password: `karan`
4. Click "Connect"

**Expected:**
- ✅ Connection succeeds immediately
- ✅ No approval dialog
- ✅ Full remote access granted
- ✅ No notifications on Device A

**If connected successfully - ✅ Remote access working!**

---

### TEST 6: Test PC Restart Auto-Start

**Do this:**
1. Restart your PC (reboot)
2. After boot, open PowerShell
3. Run: `Get-Service karanRemote`
4. Check if Status shows "Running"

**If "Running" without any action from you - ✅ Auto-start working!**

---

## TROUBLESHOOTING

### Problem: Installation fails with "Admin required"

**Solution:**
1. Right-click PowerShell
2. Select "Run as Administrator"
3. Try again

### Problem: Service shows "Stopped" after installation

**Solution - Restart service:**
```powershell
Restart-Service karanRemote -Force
```

### Problem: Can't connect from remote device

**Check:**
```powershell
Get-Service karanRemote
```

Should show "Running". If stopped:
```powershell
Start-Service karanRemote
```

### Problem: Forgot the Device ID

**Solution - Run app again:**
```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```

Shows the same ID again.

### Problem: Connection password wrong

**Verify password is exactly:** `karan` (lowercase)

No other password works (it's permanent, cannot change).

---

## COMPLETE TEST CHECKLIST

Copy this and check off as you go:

### Installation Phase
- [ ] Step 1: Ran installation script as Administrator
- [ ] Step 1: Got "INSTALLATION SUCCESSFUL" message
- [ ] Step 2: Got Device ID window
- [ ] Step 2: Wrote down Device ID
- [ ] Step 2: Closed window without errors

### Verification Phase
- [ ] Test 1: `Get-Service karanRemote` shows "Running"
- [ ] Test 2: StartType shows "Automatic"
- [ ] Test 3: No tray icon visible
- [ ] Test 4: No notifications appear
- [ ] Test 5: Remote connection works with ID + password "karan"

### Advanced Phase
- [ ] Test 6: PC restart, service auto-starts
- [ ] All tests passed ✅

---

## QUICK REFERENCE COMMANDS

### Check if service is running:
```powershell
Get-Service karanRemote
```

### Start service:
```powershell
Start-Service karanRemote
```

### Stop service:
```powershell
Stop-Service karanRemote -Force
```

### Restart service:
```powershell
Restart-Service karanRemote -Force
```

### Get device ID:
```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```

### Uninstall completely:
```powershell
powershell -ExecutionPolicy Bypass -File "C:\Users\user\Desktop\RUSTDESK\install-karanremote-fixed.ps1" -Uninstall
```

---

## WHAT TO EXPECT

### ✅ Should See:
1. Installation messages during setup
2. Device ID window on first run
3. Service running in background
4. Remote connections working

### ❌ Should NOT See:
1. Tray icon
2. Notifications
3. Toasts
4. Popups
5. Any messages after window closes
6. Approval dialogs
7. Status indicators
8. Any UI

---

## FINAL TEST: Complete Silent Operation

**Complete Test:**
1. Install service ✅
2. Get device ID ✅
3. Close window ✅
4. Check: `Get-Service karanRemote` → Running ✅
5. Look at screen → See NO tray icon ✅
6. Look for notifications → See NOTHING ✅
7. Wait 5 minutes → No messages appear ✅
8. Restart PC → Service auto-starts silently ✅
9. Connect remotely → Works with password "karan" ✅
10. **All silent, no notifications** ✅

---

## SUCCESS INDICATORS

You know it's working correctly when:

✅ Service status is "Running"
✅ No tray icon visible anywhere
✅ No notifications at any time
✅ No popups when you close the window
✅ Remote devices can connect with ID + password "karan"
✅ Everything works silently in background
✅ Auto-starts after PC restart without any indication

---

## YOU'RE ALL SET!

karanRemote is ready to:
- ✅ Show ID on first run
- ✅ Work silently forever after
- ✅ Auto-start at boot
- ✅ Accept remote connections
- ✅ Provide full remote desktop control
- ✅ All with ZERO notifications

**Start with STEP 1 above and follow through all steps!**