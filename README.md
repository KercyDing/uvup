# uvup

A conda-like environment manager for [uv](https://github.com/astral-sh/uv).

## Vision

uvup aims to be a companion tool for uv, providing a familiar conda-like interface for centralized Python virtual environment management.

## Core Philosophy

- **Enhancement, not replacement**: uvup calls uv for actual work
- **User experience first**: Familiar commands and seamless activation
- **Best practices**: Following proven patterns from conda and rustup
- **Lightweight and cross-platform**: Single binary, works everywhere

## Planned Features

### MVP (v0.1.0)

- `uvup init` - Shell integration
- `uvup create <name>` - Create environments
- `uvup activate <name>` - Activate environments
- `uvup list` - List all environments
- `uvup remove <name>` - Remove environments

### Future Versions

- `uvup default <name>` - Set default environment (auto-activate on new terminal)
- `uvup undefault` - Remove default environment
- Fish and PowerShell integration
- Cross-platform distribution (Homebrew, Scoop, Winget)

## Scope

uvup focuses on **environment management only**. For package management, use uv directly:

```bash
# Environment management with uvup
uvup create myproject
uvup activate myproject

# Package management with uv
uv pip install numpy pandas
```

## License

[MIT](LICENSE)
