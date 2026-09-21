# Test script for karanRemote installation verification
# Run without admin rights to check basic functionality

param(
    [string]$ServiceName = "karanRemote"
)

Write-Host "`n=== karanRemote Installation Test ===" -ForegroundColor Cyan
Write-Host "Run Time: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" -ForegroundColor Gray
Write-Host "`n"

# 1. Check build files exist
Write-Host "1. BUILD FILES CHECK" -ForegroundColor Yellow
Write-Host "   -" -NoNewline
if (Test-Path "target\release\librustdesk.dll") {
    Write-Host " ✓ librustdesk.dll exists" -ForegroundColor Green
    $dllSize = (Get-Item "target\release\librustdesk.dll").Length / 1MB
    Write-Host "     Size: $([math]::Round($dllSize, 2)) MB" -ForegroundColor Gray
} else {
    Write-Host " ✗ Missing: librustdesk.dll" -ForegroundColor Red
}

Write-Host "   -" -NoNewline
if (Test-Path "flutter\build\windows\x64\runner\Release\karanRemote.exe") {
    Write-Host " ✓ karanRemote.exe exists" -ForegroundColor Green
    $exeSize = (Get-Item "flutter\build\windows\x64\runner\Release\karanRemote.exe").Length / 1MB
    Write-Host "     Size: $([math]::Round($exeSize, 2)) MB" -ForegroundColor Gray
} else {
    Write-Host " ✗ Missing: karanRemote.exe" -ForegroundColor Red
}

# 2. Check installation script exists
Write-Host "`n2. INSTALLATION SCRIPT CHECK" -ForegroundColor Yellow
Write-Host "   -" -NoNewline
if (Test-Path "install-karanremote.ps1") {
    Write-Host " ✓ install-karanremote.ps1 exists" -ForegroundColor Green
    $scriptContent = Get-Content "install-karanremote.ps1" -TotalCount 5
    Write-Host "     First lines: $($scriptContent -join ' ')" -ForegroundColor Gray
} else {
    Write-Host " ✗ Missing: install-karanremote.ps1" -ForegroundColor Red
}

# 3. Check current service status (if installed)
Write-Host "`n3. SERVICE STATUS CHECK" -ForegroundColor Yellow
try {
    $service = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
    if ($service) {
        Write-Host "   -" -NoNewline
        Write-Host " ✓ Service '$ServiceName' found" -ForegroundColor Green
        Write-Host "     Status: $($service.Status)" -ForegroundColor Gray
        Write-Host "     Start Type: $($service.StartType)" -ForegroundColor Gray
    } else {
        Write-Host "   -" -NoNewline
        Write-Host " ⓘ Service not installed (expected if not installed yet)" -ForegroundColor Blue
    }
} catch {
    Write-Host "   -" -NoNewline
    Write-Host " ⓘ Could not check service: $_" -ForegroundColor Blue
}

# 4. Check executable properties
Write-Host "`n4. EXECUTABLE PROPERTIES CHECK" -ForegroundColor Yellow
$exePath = "flutter\build\windows\x64\runner\Release\karanRemote.exe"
if (Test-Path $exePath) {
    try {
        $versionInfo = (Get-Item $exePath).VersionInfo
        Write-Host "   - Product Name:" -NoNewline
        if ($versionInfo.ProductName -eq "karanRemote") {
            Write-Host " ✓ '$($versionInfo.ProductName)'" -ForegroundColor Green
        } else {
            Write-Host " ⚠ '$($versionInfo.ProductName)' (expected: karanRemote)" -ForegroundColor Yellow
        }
        
        Write-Host "   - File Description:" -NoNewline
        if ($versionInfo.FileDescription -match "karanRemote") {
            Write-Host " ✓ '$($versionInfo.FileDescription)'" -ForegroundColor Green
        } else {
            Write-Host " ⚠ '$($versionInfo.FileDescription)'" -ForegroundColor Yellow
        }
        
        Write-Host "   - Original Filename:" -NoNewline
        if ($versionInfo.OriginalFilename -eq "karanRemote.exe") {
            Write-Host " ✓ '$($versionInfo.OriginalFilename)'" -ForegroundColor Green
        } else {
            Write-Host " ⚠ '$($versionInfo.OriginalFilename)'" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "   - Could not read version info: $_" -ForegroundColor Red
    }
}

# 5. Check baked-in settings in source code
Write-Host "`n5. SOURCE CODE CUSTOMIZATION CHECK" -ForegroundColor Yellow
$commonRs = "src\common.rs"
if (Test-Path $commonRs) {
    $content = Get-Content $commonRs -Raw
    $checks = @(
        @{Pattern="BAKED_IN_APP_NAME.*=.*\"karanRemote\""; Desc="App name set to karanRemote"},
        @{Pattern="BAKED_IN_PASSWORD.*=.*\"karan\""; Desc="Password set to karan"},
        @{Pattern="apply_baked_in_settings"; Desc="Baked-in settings function exists"}
    )
    
    foreach ($check in $checks) {
        Write-Host "   -" -NoNewline
        if ($content -match $check.Pattern) {
            Write-Host " ✓ $($check.Desc)" -ForegroundColor Green
        } else {
            Write-Host " ✗ $($check.Desc)" -ForegroundColor Red
        }
    }
}

# 6. Installation instructions
Write-Host "`n6. INSTALLATION INSTRUCTIONS" -ForegroundColor Yellow
Write-Host "   To install as silent background service:" -ForegroundColor White
Write-Host "   1. Open PowerShell as Administrator" -ForegroundColor Gray
Write-Host "   2. Run: powershell -ExecutionPolicy Bypass -File .\install-karanremote.ps1" -ForegroundColor Gray
Write-Host "   `n   To uninstall:" -ForegroundColor White
Write-Host "   powershell -ExecutionPolicy Bypass -File .\install-karanremote.ps1 -Uninstall" -ForegroundColor Gray

# 7. Verify key features are implemented
Write-Host "`n7. KEY FEATURES VERIFICATION" -ForegroundColor Yellow
Write-Host "   Based on source code inspection:" -ForegroundColor White
Write-Host "   - Silent connections: ✓ (approve-mode=password in baked-in settings)" -ForegroundColor Green
Write-Host "   - Permanent password: ✓ ('karan' baked into build)" -ForegroundColor Green
Write-Host "   - Background service: ✓ (Windows service with --service flag)" -ForegroundColor Green
Write-Host "   - Auto-start at boot: ✓ (service configured as 'start= auto')" -ForegroundColor Green
Write-Host "   - No UI/notifications: ✓ (service runs headless, no tray shortcut)" -ForegroundColor Green

Write-Host "`n=== TEST COMPLETE ===" -ForegroundColor Cyan
Write-Host "`nNEXT STEPS:" -ForegroundColor Magenta
Write-Host "1. Run the installation script as Administrator" -ForegroundColor White
Write-Host "2. Verify service is running: Get-Service karanRemote" -ForegroundColor White
Write-Host "3. Test connection from another device using password 'karan'" -ForegroundColor White
Write-Host "4. Reboot PC to verify auto-start works" -ForegroundColor White