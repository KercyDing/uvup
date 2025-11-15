#!/bin/sh
set -e

# uvup uninstaller script

echo "Uninstalling uvup..."

# Find uvup binary locations
FOUND=0
BINARY_PATHS=""

# Check common installation locations
for dir in "/usr/local/bin" "$HOME/.local/bin" "$HOME/.cargo/bin"; do
    if [ -f "$dir/uvup" ]; then
        BINARY_PATHS="$BINARY_PATHS $dir/uvup"
        FOUND=1
    fi
done

# Remove binary files
if [ $FOUND -eq 1 ]; then
    for path in $BINARY_PATHS; do
        echo "Removing $path..."
        if [ -w "$(dirname "$path")" ]; then
            rm -f "$path"
            echo "  Removed: $path"
        else
            sudo rm -f "$path"
            echo "  Removed: $path (required sudo)"
        fi
    done
else
    echo "Warning: uvup binary not found in common locations"
fi

# Remove data directory
UVUP_DIR="$HOME/.uvup"
if [ -d "$UVUP_DIR" ]; then
    echo ""
    echo "Found uvup data directory: $UVUP_DIR"
    printf "Do you want to remove all virtual environments? [y/N] "
    read -r response
    case "$response" in
        [yY][eE][sS]|[yY])
            rm -rf "$UVUP_DIR"
            echo "  Removed: $UVUP_DIR"
            ;;
        *)
            echo "  Kept: $UVUP_DIR"
            ;;
    esac
fi

# Detect shell and provide cleanup instructions
SHELL_NAME=$(basename "$SHELL")
SHELL_RC=""

case "$SHELL_NAME" in
    bash)
        SHELL_RC="$HOME/.bashrc"
        ;;
    zsh)
        SHELL_RC="$HOME/.zshrc"
        ;;
    fish)
        SHELL_RC="$HOME/.config/fish/config.fish"
        ;;
    *)
        SHELL_RC="$HOME/.profile"
        ;;
esac

echo ""
echo "Please manually remove uvup initialization from your shell config:"
echo "  Edit: $SHELL_RC"
echo "  Remove lines containing: eval \"\$(uvup init)\""
echo ""

# Check for uvup in shell config
if [ -f "$SHELL_RC" ] && grep -q "eval.*uvup init\|uvup init.*source" "$SHELL_RC"; then
    echo "Found uvup initialization in $SHELL_RC"
    printf "Do you want to automatically remove it? [y/N] "
    read -r response
    case "$response" in
        [yY][eE][sS]|[yY])
            cp "$SHELL_RC" "$SHELL_RC.uvup-backup"
            # Remove uvup lines
            if [ "$SHELL_NAME" = "fish" ]; then
                # Fish uses different syntax
                sed -i.tmp '/uvup init.*source/d' "$SHELL_RC"
                sed -i.tmp '/# uvup initialization/d' "$SHELL_RC"
            else
                # Bash/Zsh
                sed -i.tmp '/eval.*uvup init/d' "$SHELL_RC"
                sed -i.tmp '/# uvup initialization/d' "$SHELL_RC"
            fi
            rm -f "$SHELL_RC.tmp"
            echo "  Removed uvup initialization from $SHELL_RC"
            echo "  Backup saved to: $SHELL_RC.uvup-backup"
            ;;
        *)
            echo "  Skipped automatic removal"
            ;;
    esac
fi

# Check PATH modifications
echo ""
echo "Checking PATH for uvup directories..."
case ":$PATH:" in
    *":.local/bin:"*)
        echo "  Found ~/.local/bin in PATH (may be used by other tools)"
        ;;
esac

echo ""
echo "uvup uninstalled successfully!"
echo ""
echo "Note: Please restart your shell or run:"
echo "  source $SHELL_RC"
