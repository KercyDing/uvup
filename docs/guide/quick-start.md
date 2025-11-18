# Quick Start

This guide will walk you through creating your first environment with uvup.

## Verify Installation

After installing uvup (see [Installation](./installation)), verify it's working:

```bash
uvup --version
```

The installation script automatically runs `uvup init` to configure for all detected shells.

If you skipped the automated installer or need to reconfigure, you can run:

```bash
uvup init
```

See the [Shell Integration](#shell-integration) section below for details.

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
python script.py        # Classic usage
uv run script.py        # uv-like usage

# Use installed tools
pytest tests/           # Classic usage
uv run pytest tests/    # uv-like usage

jupyter notebook        # Classic usage
uv run jupyter notebook # uv-like usage
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

Here's a complete example workflow from scratch:

```bash
# Create a data science environment
$ uvup create data-analysis --python 3.11
Environment 'data-analysis' created successfully

# List environments
$ uvup list
data-analysis

# Activate it
$ uvup activate data-analysis
(data-analysis) $

# Add packages
(data-analysis) $ uvup add numpy pandas matplotlib jupyter
Added: numpy, pandas, matplotlib, jupyter

# Run your analysis
(data-analysis) $ python analyze.py        # Classic usage
(data-analysis) $ uv run analyze.py        # uv-like usage

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

## Shell Integration

The `uvup init` command configures your shells to enable `uvup activate` and `uvup deactivate` commands.

### Supported Shells

| Platform | Auto-detected Shells |
|----------|---------------------|
| **Windows** | PowerShell, Git Bash |
| **macOS** | Bash, Zsh, Fish |
| **Linux** | Bash, Zsh, Fish |

### Configuration Files Modified

| Shell | File |
|-------|------|
| Bash | `~/.bashrc` (also creates `~/.bash_profile` on Windows) |
| Zsh | `~/.bashrc` |
| Fish | `~/.config/fish/config.fish` |
| PowerShell | `$PROFILE` |

### Manual Configuration

If you need to reconfigure or customize:

```bash
# Initialize all detected shells
uvup init

# Initialize only a specific shell
uvup init powershell
uvup init bash

# Preview changes without modifying files
uvup init --dry-run

# Get shell script for manual setup
uvup init --raw

# Remove shell integration
uvup init --reverse
```

After running `uvup init`, restart your terminal or reload your shell:

```bash
source ~/.bashrc  # Bash
source ~/.zshrc   # Zsh
source ~/.config/fish/config.fish  # Fish
# PowerShell: just restart terminal
```

## Troubleshooting

### `uvup activate` not found

This means shell integration wasn't set up properly. Run:

```bash
uvup init
```

Then restart your terminal.

### Changes not taking effect

Make sure you restarted your terminal or reloaded your shell configuration after running `uvup init`.

## Next Steps

- [Core Concepts](./core-concepts) - Understand uvup's design philosophy
- [Commands Reference](/commands/) - Complete command documentation
- [Use Cases](/use-cases/) - Real-world usage scenarios
