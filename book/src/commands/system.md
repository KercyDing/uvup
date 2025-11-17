# System Commands

System-level operations for uvup.

## init

Initialize shell integration for uvup.

### Usage

```bash
uvup init
```

### Supported Shells

- Bash
- Zsh
- Fish
- PowerShell

### What It Does

- Detects your current shell
- Outputs shell integration script
- Enables `uvup activate` and `uvup deactivate` commands

### Examples

```bash
# Add to your shell config file
eval "$(uvup init)"
```

### Shell Config Files

Add the above line to your shell's configuration file:

- **Bash**: `~/.bashrc` or `~/.bash_profile`
- **Zsh**: `~/.zshrc`
- **Fish**: `~/.config/fish/config.fish`
- **PowerShell**: `$PROFILE`

Then reload your shell:

```bash
# Bash/Zsh
source ~/.bashrc  # or ~/.zshrc

# Fish
source ~/.config/fish/config.fish

# PowerShell
. $PROFILE
```

### Notes

- Run this once after installation
- Restart shell or source config file to apply
- Safe to run multiple times (idempotent)

---

## update

Update uvup to the latest version.

### Usage

```bash
uvup update [OPTIONS]
```

### Options

- `-c, --check` - Only check for updates without installing

### Examples

```bash
# Update to latest version
uvup update

# Check if update available
uvup update --check
```

### What It Does

1. Checks GitHub releases for the latest version
2. Downloads the appropriate binary for your platform
3. Replaces the current binary in-place
4. Verifies the update was successful

### Notes

- Downloads latest release from GitHub
- Replaces current binary in-place
- Preserves all environments and configuration
- Requires internet connection
