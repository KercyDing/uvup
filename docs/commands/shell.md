# Shell

Shell integration commands and initialization.

## init

Initialize uvup shell integration.

### Usage

```bash
# Automatic setup - initialize all detected shells
uvup init

# Initialize specific shell
uvup init <shell>

# Options
uvup init --raw          # Print shell script instead of installing
uvup init --reverse      # Remove uvup initialization
uvup init --dry-run      # Preview changes without modifying files
```

### Supported Shells

| Platform | Auto-detected Shells |
|----------|---------------------|
| **Windows** | PowerShell, Git Bash |
| **macOS** | Bash, Zsh, Fish |
| **Linux** | Bash, Zsh, Fish |

### Examples

```bash
# Automatically configure all detected shells
uvup init

# Initialize only PowerShell
uvup init powershell

# Initialize only Bash
uvup init bash

# Preview what would be changed
uvup init --dry-run

# Remove shell integration
uvup init --reverse

# Get raw shell script for manual setup
uvup init --raw
```

### What It Does

**Automatic mode** (`uvup init`):
1. Detects all installed shells on your system
2. Adds initialization code to each shell's configuration file
3. Creates configuration files if they don't exist

**Configuration files modified:**

| Shell | File |
|-------|------|
| Bash | `~/.bashrc` (also creates `~/.bash_profile` on Windows) |
| Zsh | `~/.bashrc` |
| Fish | `~/.config/fish/config.fish` |
| PowerShell | `$PROFILE` |

**Manual mode** (`uvup init --raw`):
- Prints the shell hook script for the current shell
- Useful for custom setups or CI/CD environments

### After Running

You need to reload your shell configuration:

```bash
# Bash
source ~/.bashrc

# Zsh
source ~/.zshrc

# Fish
source ~/.config/fish/config.fish

# PowerShell
# Just restart your terminal
```

Or simply **restart your terminal**.

### Notes

- Run once after installation
- Safe to run multiple times (idempotent)
- Uses marked sections in config files for easy removal
- Doesn't modify existing configuration

---

## activate

Activate a virtual environment.

### Usage

```bash
uvup activate <name>
```

### Arguments

- `<name>` - Environment name to activate

### Examples

```bash
uvup activate myproject
```

### What It Does

1. Checks if the environment exists
2. Activates the `.venv` within the environment
3. Modifies shell prompt to show environment name
4. Enables package commands

### Effects

After activation:

- Shell prompt shows `(myproject)`
- `python` points to environment's Python
- `pip` and other tools use environment's packages
- Package commands (`add`, `remove`, `lock`, `tree`) are enabled
- Works from any directory

### Notes

- Requires `uvup init` to be set up first
- Only one environment can be active at a time
- Activating a new environment automatically deactivates the current one

---

## deactivate

Deactivate the current virtual environment.

### Usage

```bash
uvup deactivate
```

### Examples

```bash
uvup deactivate
```

### What It Does

1. Deactivates the current virtual environment
2. Restores original shell prompt
3. Restores original PATH
4. Disables package management commands

### Effects

After deactivation:

- Shell prompt returns to normal
- `python` points to system Python
- Package commands will fail with "No active environment" error

### Notes

- Requires `uvup init` to be set up first
- Safe to run even if no environment is active
- Does not delete or modify the environment
