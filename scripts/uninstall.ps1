$ErrorActionPreference = "Stop"

Write-Host "Uninstalling uvup..." -ForegroundColor Green

# Find and remove uvup binaries from common locations
$LOCATIONS = @(
    "$env:LOCALAPPDATA\Programs\uvup\uvup.exe",
    "$env:USERPROFILE\.cargo\bin\uvup.exe"
)

$FOUND = $false

foreach ($path in $LOCATIONS) {
    if (Test-Path $path) {
        Write-Host "Removing $path..." -ForegroundColor Yellow
        Remove-Item -Path $path -Force
        Write-Host "  Removed: $path" -ForegroundColor Green
        $FOUND = $true

        # Remove directory if empty (only for Programs\uvup)
        $dir = Split-Path $path -Parent
        if ($dir -like "*Programs\uvup" -and (Test-Path $dir)) {
            if ((Get-ChildItem -Path $dir -Force | Measure-Object).Count -eq 0) {
                Remove-Item -Path $dir -Force
                Write-Host "  Removed directory: $dir" -ForegroundColor Green
            }
        }
    }
}

if (-not $FOUND) {
    Write-Host "Warning: uvup binary not found in common locations" -ForegroundColor Yellow
}

# Remove data directory
$UVUP_DIR = "$env:USERPROFILE\.uvup"
if (Test-Path $UVUP_DIR) {
    Write-Host ""
    Write-Host "Found uvup data directory: $UVUP_DIR" -ForegroundColor Cyan
    $response = Read-Host "Do you want to remove all virtual environments? [y/N]"
    if ($response -match '^[yY]') {
        Remove-Item -Path $UVUP_DIR -Recurse -Force
        Write-Host "  Removed: $UVUP_DIR" -ForegroundColor Green
    } else {
        Write-Host "  Kept: $UVUP_DIR" -ForegroundColor Yellow
    }
}

# Remove from PATH
$INSTALL_DIR = "$env:LOCALAPPDATA\Programs\uvup"
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -like "*$INSTALL_DIR*") {
    Write-Host ""
    Write-Host "Removing $INSTALL_DIR from PATH..." -ForegroundColor Yellow

    $pathArray = $userPath -split ';' | Where-Object { $_ -ne $INSTALL_DIR -and $_ -ne "" }
    $newPath = $pathArray -join ';'

    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    $env:Path = $env:Path -replace [regex]::Escape(";$INSTALL_DIR"), ""
    $env:Path = $env:Path -replace [regex]::Escape("$INSTALL_DIR;"), ""

    Write-Host "  Removed from PATH" -ForegroundColor Green
}

# Remove from PowerShell profile
# Only check profile.ps1 (CurrentUserAllHosts) since install.ps1 only writes there
$PROFILE_PATH = $PROFILE.CurrentUserAllHosts
if (-not $PROFILE_PATH) {
    $PROFILE_PATH = "$env:USERPROFILE\Documents\PowerShell\profile.ps1"
}

if (Test-Path $PROFILE_PATH) {
    $profileContent = Get-Content $PROFILE_PATH -Raw -ErrorAction SilentlyContinue
    if ($profileContent -and $profileContent -match 'uvup init') {
        Write-Host ""
        Write-Host "Removing uvup from PowerShell profile..." -ForegroundColor Yellow

        # Read as lines and filter out uvup-related lines
        $lines = Get-Content $PROFILE_PATH
        $filteredLines = $lines | Where-Object {
            $_ -notmatch 'uvup init' -and
            $_ -notmatch '^\s*#\s*uvup initialization\s*$'
        }

        # Write back
        $filteredLines | Out-File -FilePath $PROFILE_PATH -Encoding utf8 -Force

        Write-Host "  Removed uvup initialization from profile" -ForegroundColor Green
    }
}

Write-Host ""
Write-Host "Please restart your terminal to continue." -ForegroundColor Cyan
