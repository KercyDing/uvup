# Core Concepts

Understanding uvup's design philosophy and architecture.

## Design Philosophy

### Enhancement, Not Replacement

uvup doesn't replace uv - it enhances it:

```bash
# uvup calls uv internally
uvup create myenv    # -> uv venv ~/.uvup/myenv/.venv
uvup add requests    # -> uv --project ~/.uvup/myenv add requests
```

**Why?** Because uv is already excellent at virtual environment and package management. uvup provides:
- Centralized environment storage
- Conda-like activation workflow
- Template-based project creation

### Familiar Interface

If you're coming from conda, uvup feels natural:

```bash
# conda workflow
conda create -n myenv python=3.11
conda activate myenv
conda install numpy pandas

# uvup workflow
uvup create myenv --python 3.11
uvup activate myenv
uvup add numpy pandas
```

### Single Binary

uvup is distributed as a single executable:
- No runtime dependencies (except uv)
- Cross-platform support (Linux, macOS, Windows)
- Easy installation and updates

### Template-Driven

Create reusable project configurations:

```bash
# Create a template environment
uvup create web-template
uvup activate web-template
uvup add fastapi uvicorn sqlalchemy pydantic

# Use it to create new projects
uvup new myapp --template web-template
uvup new another-app --template web-template
```

## Architecture

### Directory Structure

```
~/.uvup/
├── bin/
│   └── uvup              # The uvup binary
└── <env-name>/
    ├── .venv/            # Python virtual environment
    ├── pyproject.toml    # Project dependencies
    └── uv.lock           # Lock file
```

### Environment Activation

When you run `uvup activate myenv`:

1. Shell function checks if environment exists
2. Activates `.venv/bin/activate` (or Windows equivalent)
3. Sets `UVUP_ACTIVE_ENV=myenv` environment variable
4. Modifies shell prompt to show `(myenv)`

The `UVUP_ACTIVE_ENV` variable enables location-independent package management.

### Package Management Flow

```bash
$ uvup activate myenv
$ uvup add requests
```

Internally, uvup:

1. Reads `UVUP_ACTIVE_ENV` → gets `myenv`
2. Resolves path → `~/.uvup/myenv`
3. Executes: `uv --project ~/.uvup/myenv add requests`

This works from any directory because of the `--project` flag.

## Command Categories

uvup provides four distinct workflows:

### 1. Environment Management

Basic CRUD operations for environments:

```bash
uvup create myenv       # Create empty environment
uvup list               # List all environments
uvup delete myenv       # Delete environment
```

### 2. Environment Cloning

Create exact 1:1 copies:

```bash
uvup clone source target
```

Copies:
- Python version
- All dependencies (from `pyproject.toml`)
- Lock file (`uv.lock`)
- Virtual environment

Use cases:
- Backup before experiments
- Create identical environments for testing

### 3. Project Creation from Templates

Create new projects with modifications:

```bash
uvup new myapp --template web-template \
  --python 3.11 \
  --exclude pytest \
  --dry-run
```

Features:
- Python version override
- Package filtering (include/exclude)
- Custom target directory
- Dry-run preview

### 4. Project Synchronization

Update existing projects from templates:

```bash
cd myproject
uvup sync --template web-template \
  --python 3.11 \
  --dry-run
```

Features:
- Automatic backup before sync
- Rollback on failure
- Same modification options as `new`

## Dependency Management

### pyproject.toml-based

uvup uses modern Python packaging standards:

```toml
[project]
name = "myproject"
version = "0.1.0"
dependencies = [
    "numpy>=1.24.0",
    "pandas>=2.0.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0.0",
    "black>=23.0.0",
]
```

### Lock File

`uv.lock` ensures reproducible installs across machines.

### Optional Dependencies

Support for development, testing, and documentation dependencies:

```bash
# Add to main dependencies
uvup add requests

# Add to dev dependencies
uvup add --group dev pytest

# Add to docs dependencies
uvup add --group docs sphinx
```

## Shell Integration

uvup integrates deeply with your shell:

### Activation Function

The `uvup init` command generates a shell function that handles:
- Environment activation
- Deactivation
- Environment variable management
- Prompt modification

### Supported Shells

- **Bash** - Linux, macOS, Windows (Git Bash)
- **Zsh** - macOS, Linux
- **Fish** - Linux, macOS
- **PowerShell** - Windows

## Comparison with Other Tools

### vs conda

**Similarities:**
- Centralized environment management
- Activate/deactivate workflow
- List/remove commands

**Differences:**
- uvup uses uv (faster)
- uvup is Python-only
- uvup uses pyproject.toml instead of environment.yml
- uvup has template-based project creation

### vs virtualenv/venv

**Advantages of uvup:**
- Centralized storage (not per-project)
- Conda-like commands
- Template system
- Integrated package management after activation

**When to use venv:**
- Simple single-project needs
- No template requirements
- Prefer per-project .venv directories

### vs uv directly

**What uvup adds:**
- Centralized environment storage
- Shell activation integration
- Template-based workflows
- Conda-like command interface

**When to use uv directly:**
- Per-project workflows
- CI/CD pipelines
- No need for activation

## Best Practices

### Environment Naming

Use descriptive names:

```bash
# Good
uvup create data-analysis
uvup create web-backend
uvup create ml-training

# Avoid
uvup create env1
uvup create test
uvup create tmp
```

### Template Organization

Create templates for common project types:

```bash
# Base Python
uvup create base-template
uvup activate base-template
uvup add pip setuptools wheel

# Web development
uvup create web-template
uvup activate web-template
uvup add fastapi uvicorn sqlalchemy

# Data science
uvup create ds-template
uvup activate ds-template
uvup add numpy pandas matplotlib jupyter
```

### Dependency Groups

Use optional dependencies for different purposes:

```bash
# Core dependencies
uvup add requests pydantic

# Development tools
uvup add --group dev pytest black mypy ruff

# Documentation
uvup add --group docs sphinx sphinx-rtd-theme
```

### Regular Updates

Keep your environments up to date:

```bash
uvup activate myenv
uvup lock --upgrade
```

## Next Steps

- [Installation](./installation.md) - Install uvup
- [Quick Start](./quick-start.md) - Get started with uvup
- [Commands Reference](./commands/README.md) - Detailed command documentation
