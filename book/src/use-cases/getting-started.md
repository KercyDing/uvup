# Getting Started

First steps with uvup.

## First Time Setup

Complete setup from scratch:

```bash
# 1. Install uvup
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh

# 2. Initialize shell integration
uvup init

# 3. Restart your shell or source config
source ~/.bashrc  # or ~/.zshrc, etc.

# 4. Create your first environment
uvup create my-first-project --python 3.12

# 5. Activate and use it
uvup activate my-first-project
uvup add requests numpy
python -c "import requests; print(requests.__version__)"
uvup deactivate
```

## Quick Project Creation

### Data Science Project

```bash
# Create a simple data science project
uvup create ds-project
uvup activate ds-project
uvup add numpy pandas matplotlib jupyter
jupyter notebook
```

### Web API Project

```bash
# Create a web API project
uvup create web-api
uvup activate web-api
uvup add fastapi uvicorn pydantic
uvup add --group dev pytest black

# Start development server
uvicorn main:app --reload
```

### CLI Tool Project

```bash
# Create a CLI tool project
uvup create cli-tool
uvup activate cli-tool
uvup add click rich typer
uvup add --group dev pytest

# Run your CLI
python cli.py --help
```

## Understanding the Basics

### Directory Structure

After creating an environment, uvup organizes files like this:

```
~/.uvup/my-project/
├── .venv/              # Python virtual environment
├── pyproject.toml      # Project configuration
└── uv.lock             # Lock file for reproducibility
```

### Activation Workflow

```bash
# Before activation - commands fail
$ uvup add requests
Error: No active environment

# After activation - commands work
$ uvup activate myproject
(myproject) $ uvup add requests
Added: requests

# Deactivate when done
(myproject) $ uvup deactivate
$
```

### Location Independence

One of uvup's key features:

```bash
$ uvup activate myproject
(myproject) $ cd ~/anywhere/on/your/system
(myproject) $ uvup add numpy  # Still works!
```

This works because uvup uses `uv --project ~/.uvup/myproject` internally.

## Common First-Time Questions

### Where are my environments?

All environments are stored in `~/.uvup/`:

```bash
ls ~/.uvup/
# Output: my-first-project/ ds-project/ web-api/
```

### How do I see what's installed?

```bash
uvup activate myproject
uv pip list
```

### How do I remove an environment?

```bash
uvup delete myproject
```

### Can I use multiple environments?

Yes! Create as many as you need:

```bash
uvup create project-a
uvup create project-b
uvup create project-c

# Switch between them
uvup activate project-a
# ... work ...
uvup deactivate

uvup activate project-b
# ... work ...
uvup deactivate
```

## Next Steps

Continue to:
- [Development Workflows](./development.md) - Learn advanced patterns
- [Commands Reference](../commands/README.md) - Complete command documentation
