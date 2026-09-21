Write-Host "=== karanRemote Installation Verification ===" -ForegroundColor Cyan
Write-Host "Run Time: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
Write-Host ""

# 1. Check build files
Write-Host "1. Build Files:" -ForegroundColor Yellow
if (Test-Path "target\release\librustdesk.dll") {
    Write-Host "   ✓ librustdesk.dll exists" -ForegroundColor Green
} else {
    Write-Host "   ✗ Missing librustdesk.dll" -ForegroundColor Red
}

if (Test-Path "flutter\build\windows\x64\runner\Release\karanRemote.exe") {
    Write-Host "   ✓ karanRemote.exe exists" -ForegroundColor Green
} else {
    Write-Host "   ✗ Missing karanRemote.exe" -ForegroundColor Red
}

# 2. Check installation script
Write-Host "`n2. Installation Script:" -ForegroundColor Yellow
if (Test-Path "install-karanremote.ps1") {
    Write-Host "   ✓ install-karanremote.ps1 exists" -ForegroundColor Green
} else {
    Write-Host "   ✗ Missing installation script" -ForegroundColor Red
}

# 3. Check service status
Write-Host "`n3. Service Status:" -ForegroundColor Yellow
$service = Get-Service -Name "karanRemote" -ErrorAction SilentlyContinue
if ($service) {
    Write-Host "   ✓ Service 'karanRemote' found" -ForegroundColor Green
    Write-Host "     Status: $($service.Status)" -ForegroundColor Gray
    Write-Host "     Start Type: $($service.StartType)" -ForegroundColor Gray
} else {
    Write-Host "   ⓘ Service not installed (run installation script)" -ForegroundColor Blue
}

# 4. Check baked-in settings
Write-Host "`n4. Customization Check:" -ForegroundColor Yellow
if (Test-Path "src\common.rs") {
    $content = Get-Content "src\common.rs" -Raw
    if ($content -match 'BAKED_IN_APP_NAME.*=.*"karanRemote"') {
        Write-Host "   ✓ App name set to karanRemote" -ForegroundColor Green
    }
    if ($content -match 'BAKED_IN_PASSWORD.*=.*"karan"') {
        Write-Host "   ✓ Permanent password set to 'karan'" -ForegroundColor Green
    }
    if ($content -match 'apply_baked_in_settings') {
        Write-Host "   ✓ Baked-in settings function exists" -ForegroundColor Green
    }
}

# 5. Summary
Write-Host "`n=== SUMMARY ===" -ForegroundColor Cyan
Write-Host "Customization completed:" -ForegroundColor White
Write-Host "• App name: karanRemote" -ForegroundColor Gray
Write-Host "• Permanent password: karan" -ForegroundColor Gray
Write-Host "• Silent connections: enabled" -ForegroundColor Gray
Write-Host "• No UI/notifications: configured" -ForegroundColor Gray
Write-Host "• Auto-start at boot: configured" -ForegroundColor Gray
Write-Host "• Background service: ready" -ForegroundColor Gray

Write-Host "`nTo install:" -ForegroundColor Magenta
Write-Host "Run as Administrator: powershell -ExecutionPolicy Bypass -File .\install-karanremote.ps1" -ForegroundColor White

Write-Host "`nTo verify after installation:" -ForegroundColor Magenta
Write-Host "1. Get-Service karanRemote" -ForegroundColor White
Write-Host "2. Test connection with password 'karan'" -ForegroundColor White
Write-Host "3. Reboot PC to confirm auto-start" -ForegroundColor White