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
    Write-Host "PATH updated." -ForegroundColor Green
}

Write-Host ""
Write-Host "uvup installed successfully!" -ForegroundColor Green

Write-Host ""
Write-Host "Configuring shell integration..." -ForegroundColor Cyan

# Run uvup init to configure all shells
try {
    & uvup init
    Write-Host ""
    Write-Host "Shell integration configured successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Please restart your terminal to start using uvup." -ForegroundColor Cyan
} catch {
    Write-Host "Warning: Could not run uvup init" -ForegroundColor Yellow
    Write-Host "Please run 'uvup init' manually after restarting your terminal." -ForegroundColor Yellow
}

Write-Host ""
