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

**After installation, initialize your shell:**
```bash
# Automatically configure all detected shells
uvup init
```

See the [Installation Guide](https://kercyding.github.io/uvup/guide/installation) for more details.

## Quick Start

```bash
# 1. Initialize shell integration (first time only)
uvup init

# 2. Restart your terminal or reload configuration
# For Bash: source ~/.bashrc
# For Zsh: source ~/.zshrc
# For Fish: source ~/.config/fish/config.fish
# For PowerShell: restart terminal

# 3. Create and activate environment
uvup create myproject
uvup activate myproject

# 4. Add packages
uvup add numpy pandas

# 5. Work with your code
python script.py   # Classic usage
uv run script.py   # uv-like usage

# 6. Deactivate
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

- [Installation](https://kercyding.github.io/uvup/guide/installation) - Detailed setup instructions
- [Quick Start](https://kercyding.github.io/uvup/guide/quick-start) - Get started in minutes
- [Core Concepts](https://kercyding.github.io/uvup/guide/core-concepts) - Design philosophy
- [Commands](https://kercyding.github.io/uvup/commands/) - All commands with examples
- [Use Cases](https://kercyding.github.io/uvup/use-cases/) - Real-world workflows

## IDE Integration

**VSCode** - Add to your `settings.json`:
```json
{
  "python.venvPath": "~/.uvup"
}
```

## License

[MIT](LICENSE)
