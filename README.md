# uvup

[![DeepWiki](https://img.shields.io/badge/DeepWiki-blue)](https://deepwiki.com/KercyDing/uvup) [![uv](https://img.shields.io/badge/uv-black?logo=github)](https://github.com/astral-sh/uv)

A conda-like environment manager for [uv](https://github.com/astral-sh/uv).

## Vision

uvup aims to be a companion tool for uv, providing a familiar conda-like interface for centralized Python virtual environment management.

## Core Philosophy

- **Enhancement, not replacement**: uvup calls uv for actual work
- **User experience first**: Familiar commands and seamless activation
- **Best practices**: Following proven patterns from conda and rustup
- **Lightweight and cross-platform**: Single binary, works everywhere

## Installation

**Linux and macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
```

**Windows (PowerShell):**
```powershell
Invoke-RestMethod https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.ps1 | Invoke-Expression
```

For detailed installation instructions, manual installation, and developer setup, see [Installation Guide](docs/INSTALL.md).

## Uninstallation

**Linux and macOS:**
```bash
curl -fsSL -O https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/uninstall.sh
chmod +x uninstall.sh
./uninstall.sh
```

**Windows (PowerShell):**
```powershell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/uninstall.ps1 -OutFile uninstall.ps1
.\uninstall.ps1
```

For detailed uninstallation instructions and manual removal, see [Uninstallation Guide](docs/UNINSTALL.md).

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
