# Installation Guide

This guide provides detailed installation instructions for uvup across different platforms.

## Quick Install (Recommended)

### Linux and macOS

```bash
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
```

The installation script will automatically configure shell integration for you. Restart your terminal or run:

```bash
source ~/.zshrc  # or ~/.bashrc for bash
```

### Windows (PowerShell)

```powershell
Invoke-RestMethod https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.ps1 | Invoke-Expression
```

Restart your terminal or run:

```powershell
. $PROFILE
```

## Manual Installation

Download the latest release for your platform from [GitHub Releases](https://github.com/KercyDing/uvup/releases).

### Linux

```bash
# Download and install
wget https://github.com/KercyDing/uvup/releases/latest/download/uvup-linux-x86_64
chmod +x uvup-linux-x86_64
sudo mv uvup-linux-x86_64 /usr/local/bin/uvup

# Initialize shell integration
echo 'eval "$(uvup init)"' >> ~/.bashrc  # or ~/.zshrc for zsh
source ~/.bashrc
```

### macOS

**Apple Silicon:**
```bash
# Download and install
wget https://github.com/KercyDing/uvup/releases/latest/download/uvup-macos-arm64
chmod +x uvup-macos-arm64
sudo mv uvup-macos-arm64 /usr/local/bin/uvup

# Initialize shell integration
echo 'eval "$(uvup init)"' >> ~/.zshrc
source ~/.zshrc
```

**Intel Macs:**
```bash
# Download and install
wget https://github.com/KercyDing/uvup/releases/latest/download/uvup-macos-x86_64
chmod +x uvup-macos-x86_64
sudo mv uvup-macos-x86_64 /usr/local/bin/uvup

# Initialize shell integration
echo 'eval "$(uvup init)"' >> ~/.zshrc
source ~/.zshrc
```

### Windows

1. Download [uvup-windows-x86_64.exe](https://github.com/KercyDing/uvup/releases/latest/download/uvup-windows-x86_64.exe)

2. Create directory and move the binary:
   ```powershell
   New-Item -ItemType Directory -Force -Path "$env:LOCALAPPDATA\Programs\uvup"
   Move-Item uvup-windows-x86_64.exe "$env:LOCALAPPDATA\Programs\uvup\uvup.exe"
   ```

3. Add to PATH:
   - Press `Win + R`, type `sysdm.cpl`, press Enter
   - Go to "Advanced" tab → "Environment Variables"
   - Under "User variables", select "Path" and click "Edit"
   - Click "New" and add: `%LOCALAPPDATA%\Programs\uvup`
   - Click OK to save

4. Initialize shell integration (restart terminal first):
   ```powershell
   # Add to profile for all PowerShell hosts
   Add-Content -Path $PROFILE.CurrentUserAllHosts -Value "`nInvoke-Expression ((uvup init) -join `"``n`")"

   # Load in current session
   Invoke-Expression ((uvup init) -join "`n")
   ```

## For Developers

### Installing from Source

1. Clone the repository:
```bash
git clone https://github.com/KercyDing/uvup.git
cd uvup
```

2. Install from source:
```bash
cargo install --path .
```

3. Initialize shell integration:

**Linux/macOS:**
```bash
# Add to your shell configuration
echo 'eval "$(uvup init)"' >> ~/.zshrc  # or ~/.bashrc for bash
source ~/.zshrc
```

**Windows (PowerShell):**
```powershell
# Add to profile for all PowerShell hosts
Add-Content -Path $PROFILE.CurrentUserAllHosts -Value "`nInvoke-Expression ((uvup init) -join `"``n`")"

# Load in current session
Invoke-Expression ((uvup init) -join "`n")
```

### Development Workflow

During development, you have several options:

**Option 1: Reinstall after changes (Recommended)**
```bash
# After modifying code
cargo install --path .
```

**Option 2: Use cargo run for quick testing**
```bash
cargo run -- --help
cargo run -- create test-env
cargo run -- list
```

**Option 3: Build and test manually**
```bash
cargo build
./target/debug/uvup --help
```

## Verification

After installation, verify that uvup is working:

```bash
uvup --version
```

You should see the version number displayed.

## Troubleshooting

### Command not found

If you get "command not found" after installation:

1. Make sure you've restarted your terminal or sourced your shell configuration
2. Verify that uvup is in your PATH:
   ```bash
   # Linux/macOS
   which uvup

   # Windows
   where.exe uvup
   ```

### Shell integration not working

If `uvup activate` doesn't work:

1. Check that the init line is in your shell config file
2. Restart your terminal or source the config file

For more help, please visit [GitHub Issues](https://github.com/KercyDing/uvup/issues).
