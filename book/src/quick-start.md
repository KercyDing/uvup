# Quick Start

This guide will walk you through creating your first uvup environment and managing packages.

## Basic Workflow

### 1. Create an Environment

```bash
# Create a new environment
uvup create myproject

# Or with a specific Python version
uvup create myproject --python 3.12
```

This creates a new virtual environment at `~/.uvup/myproject` with:
- A `.venv` directory containing the Python virtual environment
- A `pyproject.toml` file for dependency management
- A `uv.lock` file for reproducible installs

### 2. List Environments

```bash
uvup list
```

This shows all your created environments.

### 3. Activate an Environment

```bash
uvup activate myproject
```

After activation:
- Your shell prompt shows `(myproject)`
- Python points to the environment's Python
- You can use uvup package management commands

### 4. Add Packages

```bash
# Add packages to your environment
uvup add numpy pandas requests

# Add development dependencies
uvup add --group dev pytest black mypy
```

The packages are added to `pyproject.toml` and installed automatically.

### 5. Work with Your Code

```bash
# Run Python scripts
python script.py

# Use installed tools
pytest tests/
jupyter notebook
```

### 6. Manage Dependencies

```bash
# Update the lockfile
uvup lock

# Upgrade all packages
uvup lock --upgrade

# View dependency tree
uvup tree

# View with depth limit
uvup tree --depth 2
```

### 7. Remove Packages

```bash
# Remove a package
uvup remove pandas

# Remove from a specific group
uvup remove --group dev pytest
```

### 8. Deactivate

```bash
uvup deactivate
```

This returns your shell to its original state.

### 9. Delete an Environment

```bash
uvup delete myproject
```

This permanently removes the environment directory.

## Example Session

Here's a complete example workflow:

```bash
# Create a data science environment
$ uvup create data-analysis --python 3.11
Environment 'data-analysis' created successfully

# List environments
$ uvup list
Available environments:
  data-analysis

# Activate it
$ uvup activate data-analysis
(data-analysis) $

# Add packages
(data-analysis) $ uvup add numpy pandas matplotlib jupyter
Added: numpy, pandas, matplotlib, jupyter

# Run your analysis
(data-analysis) $ python analyze.py

# Add development tools
(data-analysis) $ uvup add --group dev pytest black
Added to dev: pytest, black

# Deactivate when done
(data-analysis) $ uvup deactivate
$

# Clean up
$ uvup delete data-analysis
Environment 'data-analysis' removed successfully
```

## Key Concepts

### Centralized Management

All environments are stored in `~/.uvup/`, not scattered across project directories.

### Activation Required for Package Management

Package commands (`add`, `remove`, `lock`, `tree`) require an activated environment:

```bash
# This works
$ uvup activate myproject
(myproject) $ uvup add requests

# This fails
$ uvup add requests
Error: No active environment
```

### Location Independence

After activation, you can use uvup package commands from any directory:

```bash
$ uvup activate myproject
(myproject) $ cd ~/work/some-other-project
(myproject) $ uvup add numpy  # Still works!
```

This is powered by `uv --project <path>` internally.

## Next Steps

- [Core Concepts](./core-concepts.md) - Understand uvup's design philosophy
- [Commands Reference](./commands/README.md) - Complete command documentation
- [Use Cases](./use-cases/README.md) - Real-world usage scenarios
