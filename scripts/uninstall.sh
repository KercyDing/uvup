#!/bin/sh
set -e

# uvup uninstaller script

# Parse arguments
SKIP_CONFIRM=0
for arg in "$@"; do
    case "$arg" in
        -y|--yes)
            SKIP_CONFIRM=1
            ;;
    esac
done

echo "Uninstalling uvup..."

# Step 1: Remove shell integration using uvup init --reverse
if command -v uvup >/dev/null 2>&1; then
    echo ""
    echo "Removing shell integration..."
    uvup init --reverse
    echo "Shell integration removed"
else
    echo "Warning: uvup command not found, skipping shell integration removal"
fi

# Step 2: Ask about removing environments
UVUP_DIR="$HOME/.uvup"
if [ -d "$UVUP_DIR" ]; then
    echo ""
    echo "Found uvup data directory: $UVUP_DIR"

    if [ $SKIP_CONFIRM -eq 1 ]; then
        rm -rf "$UVUP_DIR"
        echo "Removed: $UVUP_DIR"
    else
        printf "Do you want to remove all virtual environments? [y/N] "
        read -r response
        case "$response" in
            [yY][eE][sS]|[yY])
                rm -rf "$UVUP_DIR"
                echo "Removed: $UVUP_DIR"
                ;;
            *)
                echo "Kept: $UVUP_DIR"
                echo "Note: The binary will still be removed, but environments will be preserved"
                ;;
        esac
    fi
fi

# Step 3: Find and remove binary files
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
    echo ""
    echo "Removing uvup binary..."
    for path in $BINARY_PATHS; do
        if [ -w "$(dirname "$path")" ]; then
            rm -f "$path"
            echo "Removed: $path"
        else
            sudo rm -f "$path"
            echo "Removed: $path (required sudo)"
        fi
    done
else
    echo ""
    echo "Warning: uvup binary not found in common locations"
fi

echo ""
echo "uvup uninstalled successfully!"
echo ""
echo "Please restart your terminal for changes to take effect."
echo ""
