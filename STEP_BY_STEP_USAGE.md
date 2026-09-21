# karanRemote - STEP BY STEP USAGE GUIDE

---

## 🎯 BEFORE YOU START

✅ Make sure you have:
- Windows machine with karanRemote installed
- Another device with RustDesk client (Windows, Mac, Linux, iOS, Android)
- Network connection between devices
- Password: `karan`

---

## 🚀 METHOD 1: Launch Server Manually

### On the SERVER machine (the one you want to control):

#### Step 1: Open PowerShell
```powershell
# Press: Win + R
# Type: powershell
# Press: Enter
```

#### Step 2: Run karanRemote
```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
```

#### Step 3: Note Your Device ID
A window will open showing:
```
Device ID: 123456789
```
**Write down this number!** You'll need it.

#### Step 4: Leave Server Running
Keep this window open. The service runs in background too.

---

## 🖥️ METHOD 2: Service Already Running

If the service is already running (auto-start at boot), you can check:

```powershell
# Check if service is running
Get-Service karanRemote

# Output should show:
# Status   Name
# ------   ----
# Running  karanRemote
```

To get the Device ID:
```powershell
& 'C:\Program Files\karanRemote\karanRemote.exe'
# This will show the Device ID
# Close when done
```

---

## 🌐 On the CLIENT machine (the one connecting)

### Step 1: Open RustDesk Client
- Download from: https://rustdesk.com/
- Install and launch

### Step 2: Enter Connection Details
- **ID:** `123456789` (from Step 3 above - YOUR Device ID)
- **Password:** `karan`

### Step 3: Click Connect
- Button says "Connect" or "CONNECT"
- **✅ IMPORTANT:** No dialog should appear asking for permission!
- Desktop should appear within 2-3 seconds

### Step 4: Control the Server
You can now:
- **Move Mouse** → Move it on the client device
- **Click Mouse** → Clicks happen on the server
- **Type Keyboard** → Text appears on server
- **Copy/Paste** → Works between devices

### Step 5: Disconnect
- Click the "Disconnect" button
- Connection ends cleanly
- **✅ No notifications appear**

---

## 📱 PLATFORM-SPECIFIC STEPS

### Windows Client → Windows Server
1. Download RustDesk (Windows version)
2. Install
3. Enter Device ID + Password
4. Click Connect
5. Enjoy!

### Mac Client → Windows Server
1. Download RustDesk (macOS version)
2. Install
3. Enter Device ID + Password
4. Click Connect
5. Enjoy!

### Linux Client → Windows Server
1. Download RustDesk (Linux version)
2. Install
3. Enter Device ID + Password
4. Click Connect
5. Enjoy!

### Android Client → Windows Server
1. Download RustDesk (Android app from Play Store)
2. Install
3. Click "+" to add connection
4. Enter Device ID + Password
5. Click Connect
6. Enjoy!

### iPhone Client → Windows Server
1. Download RustDesk (iOS app from App Store)
2. Install
3. Click "+" to add connection
4. Enter Device ID + Password
5. Click Connect
6. Enjoy!

---

## ✨ WHAT HAPPENS STEP BY STEP

### On Server Side (Windows with karanRemote)

**T+0s:** Server is running, waiting
```
Device ID: 123456789
[Waiting for connections]
```

**T+5s:** Client connects and sends password
```
Device ID: 123456789
[Client connecting: checking password...]
```

**T+6s:** Password accepted, connection established
```
Device ID: 123456789
[Client connected - SILENTLY]
✅ NO DIALOG APPEARS
✅ NO NOTIFICATION APPEARS
✅ Connection granted silently
```

**T+30s:** Remote user controls desktop
```
Device ID: 123456789
[Connection active - mouse/keyboard control happening]
```

**T+60s:** Client disconnects
```
Device ID: 123456789
[Waiting for connections]
✅ CLEAN DISCONNECT
✅ NO NOTIFICATION
```

---

## 🎮 COMMON TASKS & HOW TO DO THEM

### Task 1: Transfer a File
**On Client (connecting device):**
1. Look for "File Transfer" option in RustDesk
2. Click "File Transfer"
3. Select file from your computer
4. File transfers to the server desktop
5. ✅ No "approve file transfer" dialog appears

### Task 2: Copy Text Between Computers
**From Server → Client:**
1. On server, copy text (Ctrl+C)
2. On client, paste (Ctrl+V)
3. ✅ Text appears on client

**From Client → Server:**
1. On client, copy text (Ctrl+C)
2. On server, paste (Ctrl+V)
3. ✅ Text appears on server

### Task 3: Run Program on Server
1. Move mouse to server screen
2. Double-click program icon
3. ✅ Program runs on server
4. ✅ You can use it from client device

### Task 4: Multiple Simultaneous Users
**If 2nd device wants to connect:**
1. Client 2 enters Device ID + password
2. Click Connect
3. ✅ Both clients have control (sharing)
4. ✅ No "another user connected" notifications

### Task 5: Lock Server (Ctrl+Alt+Del)
1. Press Ctrl+Alt+Del on client
2. ✅ Sends to server
3. Server lock screen appears
4. Enter password on server to unlock

### Task 6: Keyboard Shortcuts
| Shortcut | What Happens |
|----------|--------------|
| Ctrl+Alt+Del | Send to server |
| Win+L | Lock server |
| Alt+Tab | Switch apps on server |
| Win+D | Show desktop on server |

---

## 🔍 VERIFICATION CHECKLIST

### Before Testing
- [ ] karanRemote service is running
- [ ] Device ID is visible
- [ ] RustDesk client is installed
- [ ] Both devices on same network (or connected via internet)

### During Connection
- [ ] No "allow remote access" dialog appears
- [ ] No permission request window
- [ ] No toast notifications
- [ ] Desktop appears quickly (2-3 seconds)
- [ ] Mouse moves when you move mouse on client
- [ ] Keyboard input works

### After Testing
- [ ] Disconnect button works
- [ ] Connection ends cleanly
- [ ] No error messages
- [ ] No notifications about disconnection
- [ ] Can reconnect immediately

---

## ❌ TROUBLESHOOTING DURING USE

### Problem: "Connection Refused"
```
Solution:
1. Verify Device ID is correct (no typos)
2. Make sure server Device ID matches what you entered
3. Check service is running: Get-Service karanRemote
4. Restart service: Restart-Service karanRemote
```

### Problem: "Wrong Password"
```
Solution:
1. Password is: karan (lowercase, no spaces)
2. Verify caps lock is off
3. Copy-paste the password to avoid typos
4. Try: karan
```

### Problem: Connection Takes Too Long
```
Solution:
1. Check internet connection
2. Try connecting again
3. If still slow, service might be busy
4. Restart: Restart-Service karanRemote
```

### Problem: Mouse Doesn't Move
```
Solution:
1. Check connection is still active
2. Try moving mouse again
3. Try clicking somewhere on remote desktop first
4. Disconnect and reconnect
```

### Problem: Keyboard Not Working
```
Solution:
1. Click on remote desktop area first (to focus)
2. Try typing again
3. Try Alt+Tab to switch windows
4. If still broken: disconnect and reconnect
```

### Problem: File Transfer Fails
```
Solution:
1. Check file is not too large
2. Check destination folder has write permissions
3. Try again
4. If persistent, restart service: Restart-Service karanRemote
```

### Problem: Getting Disconnected Randomly
```
Solution:
1. Check internet connection stability
2. Move closer to WiFi router
3. Check for network issues
4. Restart: Restart-Service karanRemote
```

---

## 🎯 EXAMPLE SCENARIOS

### Scenario 1: Remote Tech Support
**What you want:** Support person controls your computer to fix issue

**Steps:**
1. You: Run karanRemote, note Device ID
2. You: Give Device ID + password "karan" to support person
3. Support person: Open RustDesk, enter ID + password
4. Support person: Clicks Connect
5. Support person: ✅ **NO dialog appears** - instant connection
6. Support person: Fixes your computer
7. You: Click Disconnect
8. ✅ **NO notifications** throughout

### Scenario 2: Remote Work
**What you want:** Work on your office computer from home

**Steps:**
1. Office: karanRemote is always running
2. Home: Open RustDesk, enter office ID + password
3. Home: ✅ **NO dialog** - instant connection to office desktop
4. Home: Work all day on office files
5. Home: Disconnect when done
6. Office: ✅ **NO notifications** that someone was controlling

### Scenario 3: Multi-Device Support
**What you want:** Support one office computer from multiple locations

**Setup:**
- Office Computer: karanRemote running (Device ID: 111111111)
- Support Person 1: Connects when needed
- Support Person 2: Connects when needed
- Multiple can be connected at same time
- ✅ **NO notifications** for each connection

---

## 🚀 QUICK COMMANDS REFERENCE

```powershell
# Check if running
Get-Service karanRemote

# Start if not running
Start-Service karanRemote

# Stop service
Stop-Service karanRemote

# Restart service
Restart-Service karanRemote

# Launch and show Device ID
& 'C:\Program Files\karanRemote\karanRemote.exe'

# Check DLL is deployed
Get-ChildItem "C:\Program Files\karanRemote\librustdesk.dll"
```

---

## 📊 PERFORMANCE EXPECTATIONS

| Metric | Expected Value |
|--------|-----------------|
| Connection Time | 1-3 seconds |
| Mouse Response | Instant (<100ms) |
| Keyboard Response | Instant (<100ms) |
| File Transfer Speed | Network speed dependent |
| Multiple Users | Yes, supported |
| Auto-reconnect | Yes |
| Concurrent Sessions | Multiple supported |
| Notifications | ZERO |

---

## ✅ FINAL CHECKLIST FOR SUCCESSFUL USE

- [ ] Device ID is visible on startup
- [ ] Connection accepts password instantly
- [ ] **NO dialog asks for permission**
- [ ] **NO notifications appear**
- [ ] Desktop displays quickly
- [ ] Mouse control works
- [ ] Keyboard input works
- [ ] File transfer works
- [ ] Multiple connections possible
- [ ] Disconnect is clean
- [ ] Service restarts without issues
- [ ] ✅ **ZERO notifications at any point**

---

## 🎉 YOU'RE READY!

Your karanRemote Zero Notifications build is:
- ✅ Installed
- ✅ Running
- ✅ Deployed
- ✅ Ready to use

**Start using it now!**

1. Get Device ID: `& 'C:\Program Files\karanRemote\karanRemote.exe'`
2. Share ID + password "karan" with anyone
3. They connect instantly - ✅ NO notifications!
4. Enjoy silent remote desktop control

---

**Questions?** Check:
- QUICK_REFERENCE.txt - One-page overview
- TESTING_GUIDE.md - Detailed test scenarios
- ZERO_NOTIFICATIONS_COMPLETE.md - Technical details

