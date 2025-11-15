# Uninstallation Guide

This guide provides detailed instructions for uninstalling uvup from your system.

## Quick Uninstall (Recommended)

### Linux and macOS

```bash
curl -fsSL -O https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/uninstall.sh
chmod +x uninstall.sh
./uninstall.sh
```

### Windows (PowerShell)

```powershell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/uninstall.ps1 -OutFile uninstall.ps1
.\uninstall.ps1
```

### What the uninstall script does:

- Remove the uvup binary
- Ask if you want to delete all virtual environments in `~/.uvup`
- Remove uvup from PATH (Windows only, if installed via install.ps1)
- Remove shell integration from your profile

## Manual Uninstall

If you prefer to uninstall manually or if the script doesn't work:

### Linux/macOS

1. Remove binary:
```bash
# If installed to /usr/local/bin
sudo rm /usr/local/bin/uvup

# OR if installed to ~/.local/bin
rm ~/.local/bin/uvup

# OR if installed via cargo
rm ~/.cargo/bin/uvup
```

2. Remove data directory (optional):
```bash
rm -rf ~/.uvup
```

3. Remove from shell config:

Edit your shell configuration file and remove lines containing `eval "$(uvup init)"`:

```bash
# For bash users
nano ~/.bashrc

# For zsh users
nano ~/.zshrc

# For fish users
nano ~/.config/fish/config.fish
```

4. Reload your shell:
```bash
source ~/.bashrc  # or ~/.zshrc
```

### Windows

1. Remove binary:
```powershell
# If installed to Programs
Remove-Item "$env:LOCALAPPDATA\Programs\uvup\uvup.exe"

# OR if installed via cargo
Remove-Item "$env:USERPROFILE\.cargo\bin\uvup.exe"
```

2. Remove data directory (optional):
```powershell
Remove-Item -Recurse "$env:USERPROFILE\.uvup"
```

3. Remove from PATH (if added manually):
   - Press `Win + R`, type `sysdm.cpl`, press Enter
   - Go to "Advanced" tab → "Environment Variables"
   - Under "User variables", select "Path" and click "Edit"
   - Find and remove: `%LOCALAPPDATA%\Programs\uvup`
   - Click OK to save

4. Remove from PowerShell profile:

Find your profile file:
```powershell
echo $PROFILE.CurrentUserAllHosts
```

Edit the file and remove lines containing `uvup init`.

5. Restart your PowerShell terminal.

## Removing Virtual Environments

The uninstall scripts will ask if you want to remove all virtual environments. If you chose not to remove them during uninstall, or if you want to remove them later:

### Linux/macOS
```bash
rm -rf ~/.uvup
```

### Windows
```powershell
Remove-Item -Recurse "$env:USERPROFILE\.uvup"
```

**Warning:** This will permanently delete all Python virtual environments managed by uvup.

## Verification

After uninstallation, verify that uvup is removed:

```bash
# This should return "command not found"
uvup --version
```

## Reinstallation

If you want to reinstall uvup later, please refer to the [Installation Guide](INSTALL.md).

## Troubleshooting

### uvup command still works after uninstall

This might happen if:

1. You have multiple installations (e.g., both in `/usr/local/bin` and `~/.cargo/bin`)
   - Check with: `which uvup` (Linux/macOS) or `where.exe uvup` (Windows)
   - Remove all instances

2. Your PATH cache hasn't been refreshed
   - Restart your terminal
   - Or run: `hash -r` (bash/zsh)

### Shell integration still active

If you still see uvup functions after uninstall:

1. Make sure you've removed the init line from your shell config
2. Restart your terminal or source the config file
3. Check for init lines in multiple config files (e.g., both `.bashrc` and `.bash_profile`)

For more help, please visit [GitHub Issues](https://github.com/KercyDing/uvup/issues).
