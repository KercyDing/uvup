# Core Concepts

Understanding uvup's design philosophy and key features.

## Design Philosophy

uvup enhances uv by providing:
- **Centralized environment storage** at `~/.uvup/`
- **Conda-like activation workflow** for easy environment switching
- **Template-based project creation** for reusable configurations

If you're coming from conda, uvup feels natural:

```bash
# conda workflow
conda create -n myenv python=3.11
conda activate myenv
conda install numpy

# uvup workflow
uvup create myenv --python 3.11
uvup activate myenv
uvup add numpy
```

## Key Features

### Centralized Management

All environments in `~/.uvup/`, not scattered across project directories.

### Location Independence

Manage packages from any directory after activation:

```bash
uvup activate myproject
cd ~/anywhere
uvup add requests  # Works!
```

### Template System

Reuse environment configurations:

```bash
# Create template
uvup create web-template
uvup activate web-template
uvup add fastapi uvicorn

# Use for new projects
uvup new myapp --template web-template
```

### Four Workflows

1. **Environment Management**: `create`, `list`, `delete`
2. **Cloning**: `clone source target` - exact 1:1 copies
3. **Project Creation**: `new` - from templates with customization
4. **Synchronization**: `sync` - update existing projects from templates

## Dependency Management

Uses modern Python standards:
- `pyproject.toml` for dependencies
- `uv.lock` for reproducible installs
- Optional dependency groups

```bash
uvup add requests              # Main
uvup add --group dev pytest    # Development
uvup add --group docs sphinx   # Documentation
```

## Comparison with Other Tools

| Tool | uvup Advantage | When to Use Other Tool |
|------|----------------|----------------------|
| **conda** | Faster (uses uv), Python-only | Need non-Python packages |
| **virtualenv/venv** | Centralized, templates, conda-like | Simple per-project needs |
| **uv directly** | Shell integration, templates | CI/CD, per-project workflows |

## Best Practices

**Use descriptive names:**
```bash
uvup create data-analysis  # Good
uvup create env1           # Bad
```

**Organize with templates:**
```bash
uvup create web-template
uvup create ds-template
uvup create ml-template
```

**Use dependency groups:**
```bash
uvup add requests pydantic              # Core
uvup add --group dev pytest black mypy  # Development
uvup add --group docs sphinx            # Documentation
```

## Next Steps

- [Commands Reference](/commands/) - Complete command documentation
- [Use Cases](/use-cases/) - Real-world examples
