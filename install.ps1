# Set execution policy to allow script execution (run as Administrator if needed)
# Set-ExecutionPolicy RemoteSigned -Scope CurrentUser

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
Write-Host ""
Write-Host "To complete installation, add uvup to your PowerShell profile:" -ForegroundColor Cyan
Write-Host "  uvup init | Invoke-Expression" -ForegroundColor White
Write-Host ""
Write-Host "Or run this command to initialize in current shell:" -ForegroundColor Cyan
Write-Host "  uvup init | Invoke-Expression" -ForegroundColor White
