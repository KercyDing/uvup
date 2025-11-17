# uvup

[![DeepWiki](https://img.shields.io/badge/DeepWiki-blue)](https://deepwiki.com/KercyDing/uvup) [![uv](https://img.shields.io/badge/uv-black?logo=github)](https://github.com/astral-sh/uv)

A conda-like environment manager for [uv](https://github.com/astral-sh/uv).

## Vision

uvup aims to be a companion tool for uv, providing a familiar conda-like interface for centralized Python virtual environment management.

## Core Philosophy

- **Enhancement, not replacement**: Calls uv for actual work
- **Familiar interface**: Conda-like commands, seamless activation
- **Single binary**: Lightweight and cross-platform
- **Template-driven**: Reusable project configurations

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

### v0.2.0 - Completed

- [x] `uvup clone <source> <target>` - Clone environments (1:1 exact copy)
- [x] `uvup new <name> --template <template>` - Create projects from templates
- [x] `uvup sync --template <template>` - Sync current project with template
- [x] Template modification support (--python, --exclude, --include)
- [x] Dry-run preview mode for all template operations
- [x] pyproject.toml-based dependency management
- [x] optional-dependencies support

### Future Versions

- Official template repository with curated project templates
- `uvup template list` - Browse available official templates
- Auto-download templates on first use
- `uvup default <name>` - Set default environment
- Package manager support (Homebrew, Scoop, Winget)

## Usage

### Quick Start

```bash
# Create a new environment
uvup create myproject

# Create with specific Python version
uvup create myproject --python 3.12

# List all environments
uvup list

# Activate an environment
uvup activate myproject

# Install packages (using uv)
uv add numpy pandas

# Deactivate
uvup deactivate

# Remove an environment
uvup remove myproject
```

### Environment Cloning

Clone an existing environment to create an exact 1:1 copy:

```bash
# Clone an environment
uvup clone myproject myproject-backup

# The cloned environment will have identical:
# - Python version
# - All dependencies (from pyproject.toml)
# - Lock file (uv.lock)
```

### Template-based Project Creation

Create new projects from template environments with modification support:

```bash
# Preview changes before creating
uvup new myapp --template web-template --dry-run

# Create a project from a template
uvup new myapp --template web-template

# Create with custom Python version
uvup new myapp --template web-template --python 3.11

# Create with package filtering
uvup new myapp --template web-template --exclude pytest,black
uvup new myapp --template web-template --include numpy,pandas,requests

# Create in a custom directory
uvup new myapp --template web-template --path ~/projects
```

### Template Synchronization

Sync an existing project with a template environment:

```bash
# Preview changes before syncing
uvup sync --template web-template --dry-run

# Sync current project with template
cd myproject
uvup sync --template web-template

# Sync with Python version override
uvup sync --template web-template --python 3.11

# Sync with package filtering
uvup sync --template web-template --exclude dev-packages
uvup sync --template web-template --include numpy,pandas
```

### Command Categories

uvup provides four distinct command categories:

1. **Create** - Create empty environments
   ```bash
   uvup create myenv --python 3.12
   ```

2. **Clone** - 1:1 exact copy (no modifications)
   ```bash
   uvup clone source-env target-env
   ```

3. **New** - Create projects from templates (with modifications)
   ```bash
   uvup new myproject --template base-template --exclude pytest
   ```

4. **Sync** - Update current project from template (with modifications)
   ```bash
   uvup sync --template base-template --python 3.11
   ```

For complete command reference with all options and examples, see [COMMANDS.md](docs/COMMANDS.md).

## Documentation

### Quick Help

Use the built-in help system for command-specific usage:

```bash
# General help
uvup --help

# Command-specific help
uvup new --help
uvup sync --help
uvup clone --help
```

### Complete Documentation

- **[COMMANDS.md](docs/COMMANDS.md)** - Complete command reference with all options and examples
- **[USE_CASES.md](docs/USE_CASES.md)** - Real-world usage scenarios and workflows
- **[CHANGELOG.md](CHANGELOG.md)** - Version history and release notes

## Scope

uvup focuses on **environment management and template-based workflows**. For package management, use uv directly:

```bash
# Environment management with uvup
uvup create myproject
uvup activate myproject

# Package management with uv
uv add numpy pandas
uv remove pandas
uv lock
uv sync
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
