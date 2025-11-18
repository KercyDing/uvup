# Installation

## Prerequisites

uvup requires [uv](https://github.com/astral-sh/uv) to be installed:

```bash
# Install uv if you haven't already
curl -LsSf https://astral.sh/uv/install.sh | sh

# On Windows:
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

## Quick Install

### Linux and macOS

```bash
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
```

### Windows (PowerShell)

```powershell
Invoke-RestMethod https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.ps1 | Invoke-Expression
```

## What the Installer Does

The installation script will:

1. Download the latest uvup binary for your platform
2. Install it to:
   - Linux/macOS: `~/.local/bin/uvup` (or `/usr/local/bin` if writable)
   - Windows: `%LOCALAPPDATA%\Programs\uvup\uvup.exe`
3. Add the installation directory to your PATH
4. Automatically run `uvup init` to configure shell integration

## Verify Installation

Check that uvup is installed correctly:

```bash
uvup --version
```

## Build from Source

For development or custom builds:

```bash
# Clone the repository
git clone https://github.com/KercyDing/uvup.git
cd uvup

# Build the release binary
cargo build --release

# The binary will be at target/release/uvup
# Manually copy to your PATH and run 'uvup init'
```

## Uninstall

To remove uvup from your system:

### Linux and macOS

```bash
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/uninstall.sh | sh -s -- -y
```

### Windows (PowerShell)

```powershell
Invoke-RestMethod https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/uninstall.ps1 | Invoke-Expression
```

The uninstall script will:
- Remove shell integration (`uvup init --reverse`)
- Delete the uvup binary
- Remove from PATH
- Delete all environments

### Manual Uninstall

If you prefer to uninstall manually:

```bash
# 1. Remove shell integration
uvup init --reverse

# 2. Delete all environments (optional)
rm -rf ~/.uvup  # Linux/macOS
Remove-Item -Recurse -Force "$env:USERPROFILE\.uvup"  # Windows
```

## Update

Update uvup to the latest version:

```bash
# Update to latest version
uvup update

# Check if update available
uvup update --check
```

The update command will:
- Check GitHub releases for the latest version
- Download the appropriate binary for your platform
- Replace the current binary in-place
- Preserve all environments and configuration

**Note:** Restart your terminal after updating to use the new version.

## Next Steps

Continue to [Quick Start](./quick-start) to start using uvup!
