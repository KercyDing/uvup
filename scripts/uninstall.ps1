$ErrorActionPreference = "Stop"

Write-Host "Uninstalling uvup..." -ForegroundColor Green

# Step 1: Remove shell integration using uvup init --reverse
try {
    Write-Host ""
    Write-Host "Removing shell integration..." -ForegroundColor Cyan
    & uvup init --reverse
    Write-Host "Shell integration removed" -ForegroundColor Green
} catch {
    Write-Host "Warning: Could not run uvup init --reverse" -ForegroundColor Yellow
    Write-Host "Skipping shell integration removal" -ForegroundColor Yellow
}

# Step 2: Ask about removing environments
$UVUP_DIR = "$env:USERPROFILE\.uvup"
if (Test-Path $UVUP_DIR) {
    Write-Host ""
    Write-Host "Found uvup data directory: $UVUP_DIR" -ForegroundColor Cyan
    $response = Read-Host "Do you want to remove all virtual environments? [y/N]"
    if ($response -match '^[yY]') {
        Remove-Item -Path $UVUP_DIR -Recurse -Force
        Write-Host "Removed: $UVUP_DIR" -ForegroundColor Green
    } else {
        Write-Host "Kept: $UVUP_DIR" -ForegroundColor Yellow
        Write-Host "Note: The binary will still be removed, but environments will be preserved" -ForegroundColor Yellow
    }
}

# Step 3: Find and remove uvup binaries
Write-Host ""
Write-Host "Removing uvup binary..." -ForegroundColor Cyan

$LOCATIONS = @(
    "$env:LOCALAPPDATA\Programs\uvup\uvup.exe",
    "$env:USERPROFILE\.cargo\bin\uvup.exe"
)

$FOUND = $false

foreach ($path in $LOCATIONS) {
    if (Test-Path $path) {
        Remove-Item -Path $path -Force
        Write-Host "Removed: $path" -ForegroundColor Green
        $FOUND = $true

        # Remove directory if empty (only for Programs\uvup)
        $dir = Split-Path $path -Parent
        if ($dir -like "*Programs\uvup" -and (Test-Path $dir)) {
            if ((Get-ChildItem -Path $dir -Force | Measure-Object).Count -eq 0) {
                Remove-Item -Path $dir -Force
                Write-Host "Removed directory: $dir" -ForegroundColor Green
            }
        }
    }
}

if (-not $FOUND) {
    Write-Host "Warning: uvup binary not found in common locations" -ForegroundColor Yellow
}

# Step 4: Remove from PATH
$INSTALL_DIR = "$env:LOCALAPPDATA\Programs\uvup"
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -like "*$INSTALL_DIR*") {
    Write-Host ""
    Write-Host "Removing from PATH..." -ForegroundColor Cyan

    $pathArray = $userPath -split ';' | Where-Object { $_ -ne $INSTALL_DIR -and $_ -ne "" }
    $newPath = $pathArray -join ';'

    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    Write-Host "Removed from PATH" -ForegroundColor Green
}

Write-Host ""
Write-Host "uvup uninstalled successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Please restart your terminal for changes to take effect." -ForegroundColor Cyan
Write-Host ""
