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
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/install.sh | sh
```

**Windows (PowerShell):**
```powershell
Invoke-RestMethod https://raw.githubusercontent.com/KercyDing/uvup/main/install.ps1 | Invoke-Expression
```

After installation, initialize shell integration:
```bash
# Linux/macOS (Bash/Zsh)
eval "$(uvup init)"

# Windows (PowerShell)
uvup init | Invoke-Expression
```

### Manual Installation

Download the latest release for your platform from [GitHub Releases](https://github.com/KercyDing/uvup/releases):

**Linux:**
```bash
wget https://github.com/KercyDing/uvup/releases/latest/download/uvup-linux-x86_64
chmod +x uvup-linux-x86_64
sudo mv uvup-linux-x86_64 /usr/local/bin/uvup
```

**macOS:**
```bash
# For Apple Silicon (M1/M2/M3)
wget https://github.com/KercyDing/uvup/releases/latest/download/uvup-macos-arm64
chmod +x uvup-macos-arm64
sudo mv uvup-macos-arm64 /usr/local/bin/uvup

# For Intel Macs
wget https://github.com/KercyDing/uvup/releases/latest/download/uvup-macos-x86_64
chmod +x uvup-macos-x86_64
sudo mv uvup-macos-x86_64 /usr/local/bin/uvup
```

**Windows:**
1. Download [uvup-windows-x86_64.exe](https://github.com/KercyDing/uvup/releases/latest/download/uvup-windows-x86_64.exe)
2. Rename to `uvup.exe`
3. Move to a directory in your PATH (e.g., `C:\Program Files\uvup\`)

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
```bash
# Add to your shell configuration
echo 'eval "$(uvup init)"' >> ~/.zshrc  # or ~/.bashrc for bash
source ~/.zshrc
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
