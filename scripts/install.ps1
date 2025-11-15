# Check execution policy
$executionPolicy = Get-ExecutionPolicy -Scope CurrentUser
if ($executionPolicy -eq "Restricted" -or $executionPolicy -eq "Undefined") {
    Write-Host "Error: PowerShell script execution is disabled on this system." -ForegroundColor Red
    Write-Host ""
    Write-Host "To allow script execution, run the following command:" -ForegroundColor Yellow
    Write-Host "  Set-ExecutionPolicy RemoteSigned -Scope CurrentUser" -ForegroundColor White
    Write-Host ""
    Write-Host "Then run this installation script again." -ForegroundColor Yellow
    exit 1
}

$ErrorActionPreference = "Stop"

Write-Host "Downloading uvup for Windows..." -ForegroundColor Green

$DOWNLOAD_URL = "https://github.com/KercyDing/uvup/releases/latest/download/uvup-windows-x86_64.exe"
$INSTALL_DIR = "$env:LOCALAPPDATA\Programs\uvup"
$INSTALL_PATH = "$INSTALL_DIR\uvup.exe"

# Create install directory
New-Item -ItemType Directory -Force -Path $INSTALL_DIR | Out-Null

# Download binary
try {
    Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $INSTALL_PATH -UseBasicParsing
} catch {
    Write-Host "Error: Failed to download uvup" -ForegroundColor Red
    exit 1
}

Write-Host "uvup installed to $INSTALL_PATH" -ForegroundColor Green

# Add to PATH if not already present
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$INSTALL_DIR*") {
    Write-Host "Adding $INSTALL_DIR to PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$INSTALL_DIR", "User")
    $env:Path = "$env:Path;$INSTALL_DIR"
    Write-Host "PATH updated. You may need to restart your terminal." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "uvup installed successfully!" -ForegroundColor Green

# Add uvup initialization to PowerShell profile
# Use CurrentUserAllHosts (profile.ps1) - works for all PowerShell hosts
$PROFILE_PATH = $PROFILE.CurrentUserAllHosts
if (-not $PROFILE_PATH) {
    $PROFILE_PATH = "$env:USERPROFILE\Documents\PowerShell\profile.ps1"
}

Write-Host ""
Write-Host "Configuring PowerShell profile..." -ForegroundColor Cyan

# Create profile if it doesn't exist
if (-not (Test-Path $PROFILE_PATH)) {
    $profileDir = Split-Path $PROFILE_PATH -Parent
    if (-not (Test-Path $profileDir)) {
        New-Item -ItemType Directory -Path $profileDir -Force | Out-Null
    }
    New-Item -ItemType File -Path $PROFILE_PATH -Force | Out-Null
    Write-Host "Created profile at: $PROFILE_PATH" -ForegroundColor Green
}

# Check if already exists
$profileContent = Get-Content $PROFILE_PATH -Raw -ErrorAction SilentlyContinue
if ($profileContent -match 'uvup init.*Invoke-Expression') {
    Write-Host "uvup initialization already exists in profile" -ForegroundColor Yellow
} else {
    # Add initialization line
    $initLine = 'Invoke-Expression ((uvup init) -join "`n")'
    Add-Content -Path $PROFILE_PATH -Value "`n# uvup initialization"
    Add-Content -Path $PROFILE_PATH -Value $initLine
    Write-Host "Added uvup initialization to profile" -ForegroundColor Green
}

Write-Host ""
Write-Host "To start using uvup, run:" -ForegroundColor Cyan
Write-Host "  . `$PROFILE" -ForegroundColor White
Write-Host ""
Write-Host "Or restart your terminal." -ForegroundColor Cyan
