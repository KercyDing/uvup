# uvup

[![Docs](https://img.shields.io/badge/Docs-blue)](https://kercyding.github.io/uvup/) [![DeepWiki](https://img.shields.io/badge/DeepWiki-orange)](https://deepwiki.com/KercyDing/uvup) [![uv](https://img.shields.io/badge/uv-black?logo=github)](https://github.com/astral-sh/uv)

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

See the [Installation Guide](https://kercyding.github.io/uvup/installation.html) for more details.

## Quick Start

```bash
# Create and activate environment
uvup create myproject
uvup activate myproject

# Add packages
uvup add numpy pandas

# Work with your code
python script.py

# Deactivate
uvup deactivate
```

📖 **[Read the full documentation](https://kercyding.github.io/uvup/)** for complete usage guide.

## Key Features

### Environment Management
```bash
uvup create myenv          # Create environment
uvup list                  # List all environments
uvup activate myenv        # Activate environment
uvup delete myenv          # Delete environment
```

### Template System
```bash
# Clone existing environment
uvup clone source target

# Create from template with modifications
uvup new myapp --template web-template --exclude dev-tools

# Sync current project with template
uvup sync --template web-template --dry-run
```

### Package Management
```bash
# After activation, manage packages from anywhere
uvup add requests numpy pandas
uvup remove pandas
uvup lock --upgrade
uvup tree
```

## Documentation

📖 **[Full Documentation](https://kercyding.github.io/uvup/)** - Complete user guide

- [Installation](https://kercyding.github.io/uvup/installation.html) - Detailed setup instructions
- [Quick Start](https://kercyding.github.io/uvup/quick-start.html) - Get started in minutes
- [Command Reference](https://kercyding.github.io/uvup/commands/) - All commands with examples
- [Use Cases](https://kercyding.github.io/uvup/use-cases/) - Real-world workflows
- [Core Concepts](https://kercyding.github.io/uvup/core-concepts.html) - Design philosophy

## IDE Integration

**VSCode** - Add to your `settings.json`:
```json
{
  "python.venvPath": "~/.uvup"
}
```

## License

[MIT](LICENSE)
