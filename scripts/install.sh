#!/bin/sh
set -e

# uvup installer script

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

# Determine download URL
case "$OS" in
    Linux*)
        if [ "$ARCH" = "x86_64" ]; then
            BINARY="uvup-linux-x86_64"
        else
            echo "Error: Unsupported architecture $ARCH for Linux"
            exit 1
        fi
        ;;
    Darwin*)
        if [ "$ARCH" = "arm64" ]; then
            BINARY="uvup-macos-arm64"
        elif [ "$ARCH" = "x86_64" ]; then
            BINARY="uvup-macos-x86_64"
        else
            echo "Error: Unsupported architecture $ARCH for macOS"
            exit 1
        fi
        ;;
    *)
        echo "Error: Unsupported operating system $OS"
        exit 1
        ;;
esac

DOWNLOAD_URL="https://github.com/KercyDing/uvup/releases/latest/download/$BINARY"

echo "Downloading uvup for $OS $ARCH..."
TEMP_FILE=$(mktemp)
if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$DOWNLOAD_URL" -o "$TEMP_FILE"
elif command -v wget >/dev/null 2>&1; then
    wget -q "$DOWNLOAD_URL" -O "$TEMP_FILE"
else
    echo "Error: curl or wget is required"
    exit 1
fi

# Make executable
chmod +x "$TEMP_FILE"

# Determine install directory
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
    sudo=""
elif [ "$(id -u)" -eq 0 ]; then
    INSTALL_DIR="/usr/local/bin"
    sudo=""
else
    INSTALL_DIR="$HOME/.local/bin"
    sudo=""
    mkdir -p "$INSTALL_DIR"
fi

# Move to install directory
echo "Installing uvup to $INSTALL_DIR..."
if [ -n "$sudo" ]; then
    $sudo mv "$TEMP_FILE" "$INSTALL_DIR/uvup"
else
    mv "$TEMP_FILE" "$INSTALL_DIR/uvup"
fi

echo "uvup installed successfully!"

# Check if install directory is in PATH
case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        echo ""
        echo "Warning: $INSTALL_DIR is not in your PATH"
        echo "Add the following to your shell configuration file:"
        echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
        ;;
esac

# Detect shell and provide init instructions
SHELL_NAME=$(basename "$SHELL")
SHELL_RC=""
INIT_LINE=""

case "$SHELL_NAME" in
    bash)
        SHELL_RC="$HOME/.bashrc"
        INIT_LINE='eval "$(uvup init)"'
        ;;
    zsh)
        SHELL_RC="$HOME/.zshrc"
        INIT_LINE='eval "$(uvup init)"'
        ;;
    fish)
        SHELL_RC="$HOME/.config/fish/config.fish"
        INIT_LINE='uvup init | source'
        ;;
    *)
        SHELL_RC="$HOME/.profile"
        INIT_LINE='eval "$(uvup init)"'
        ;;
esac

echo ""
echo "Configuring shell integration..."

# Check if already exists
if [ -f "$SHELL_RC" ] && grep -q "eval.*uvup init\|uvup init.*source" "$SHELL_RC"; then
    echo "uvup initialization already exists in $SHELL_RC"
else
    # Create shell RC if it doesn't exist
    touch "$SHELL_RC"

    # Add init line
    echo "" >> "$SHELL_RC"
    echo "# uvup initialization" >> "$SHELL_RC"
    echo "$INIT_LINE" >> "$SHELL_RC"

    echo "Added uvup initialization to $SHELL_RC"
fi

echo ""
echo "To start using uvup, run:"
echo "  source $SHELL_RC"
echo ""
echo "Or restart your terminal."
