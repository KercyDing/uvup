# Installation

## Quick Install

### Linux and macOS

```bash
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
```

### Windows (PowerShell)

```powershell
Invoke-RestMethod https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.ps1 | Invoke-Expression
```

## What the Installer Does

The installation script will:

1. Download the latest uvup binary for your platform
2. Install it to `~/.uvup/bin/uvup`
3. Add `~/.uvup/bin` to your PATH
4. Initialize shell integration

## Shell Integration

After installation, you need to initialize shell integration:

```bash
# Add this to your shell config file
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

## Verify Installation

Check that uvup is installed correctly:

```bash
uvup --version
```

## Prerequisites

uvup requires [uv](https://github.com/astral-sh/uv) to be installed:

```bash
# Install uv if you haven't already
curl -LsSf https://astral.sh/uv/install.sh | sh
```

## Manual Installation

If you prefer to install manually:

1. Download the binary for your platform from [GitHub Releases](https://github.com/KercyDing/uvup/releases)
2. Extract the binary to a directory in your PATH
3. Make it executable: `chmod +x uvup` (Linux/macOS)
4. Add shell integration to your shell config file

## Developer Setup

For development builds from source:

```bash
# Clone the repository
git clone https://github.com/KercyDing/uvup.git
cd uvup

# Build the project
cargo build --release

# The binary will be at target/release/uvup
```

## Next Steps

Continue to [Quick Start](./quick-start.md) to start using uvup!
