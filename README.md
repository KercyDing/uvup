# uvup

A conda-like environment manager for [uv](https://github.com/astral-sh/uv).

## Vision

uvup aims to be a companion tool for uv, providing a familiar conda-like interface for centralized Python virtual environment management.

## Core Philosophy

- **Enhancement, not replacement**: uvup calls uv for actual work
- **User experience first**: Familiar commands and seamless activation
- **Best practices**: Following proven patterns from conda and rustup
- **Lightweight and cross-platform**: Single binary, works everywhere

## Installation

### Quick Install (Recommended)

**Linux and macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
```

**Windows (PowerShell):**
```powershell
Invoke-RestMethod https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.ps1 | Invoke-Expression
```

The installation script will automatically configure shell integration for you. Restart your terminal or run:

```bash
# Linux/macOS
source ~/.zshrc  # or ~/.bashrc for bash

# Windows (PowerShell)
. $PROFILE
```

### Manual Installation

Download the latest release for your platform from [GitHub Releases](https://github.com/KercyDing/uvup/releases):

**Linux:**
```bash
# Download and install
wget https://github.com/KercyDing/uvup/releases/latest/download/uvup-linux-x86_64
chmod +x uvup-linux-x86_64
sudo mv uvup-linux-x86_64 /usr/local/bin/uvup

# Initialize shell integration
echo 'eval "$(uvup init)"' >> ~/.bashrc  # or ~/.zshrc for zsh
source ~/.bashrc
```

**macOS:**
```bash
# Download and install (Apple Silicon)
wget https://github.com/KercyDing/uvup/releases/latest/download/uvup-macos-arm64
chmod +x uvup-macos-arm64
sudo mv uvup-macos-arm64 /usr/local/bin/uvup

# OR for Intel Macs
wget https://github.com/KercyDing/uvup/releases/latest/download/uvup-macos-x86_64
chmod +x uvup-macos-x86_64
sudo mv uvup-macos-x86_64 /usr/local/bin/uvup

# Initialize shell integration
echo 'eval "$(uvup init)"' >> ~/.zshrc
source ~/.zshrc
```

**Windows:**

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

### For Developers

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

#### Development Workflow

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

## Planned Features

### MVP (v0.1.0) - Completed

- [x] `uvup init` - Shell integration (Bash, Zsh, Fish, PowerShell)
- [x] `uvup create <name>` - Create environments
- [x] `uvup activate <name>` - Activate environments (via shell hook)
- [x] `uvup list` - List all environments
- [x] `uvup remove <name>` - Remove environments

### Future Versions

- `uvup default <name>` - Set default environment (auto-activate on new terminal)
- `uvup undefault` - Remove default environment
- Installation via package managers (Homebrew, Scoop, Winget)
- Enhanced `list` command with more environment details

## Usage

### Quick Start

```bash
# Create a new environment
uvup create myproject

# Create with specific Python version
uvup create myproject --python 3.12
# or
uvup create --python 3.12 myproject

# List all environments
uvup list

# Activate an environment
uvup activate myproject

# Install packages (using uv)
uv pip install numpy pandas

# Deactivate
uvup deactivate

# Remove an environment
uvup remove myproject
```

## Scope

uvup focuses on **environment management only**. For package management, use uv directly:

```bash
# Environment management with uvup
uvup create myproject
uvup activate myproject

# Package management with uv
uv pip install numpy pandas
```

## IDE Integration

### VSCode

Add to your `settings.json` to find your venv:

```json
{
  "python.venvPath": "~/.uvup"
}
```

## License

[MIT](LICENSE)
