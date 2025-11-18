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
    *":$INSTALL_DIR:"*)
        PATH_CONFIGURED=1
        ;;
    *)
        echo ""
        echo "Adding $INSTALL_DIR to PATH..."
        # Determine shell config file
        SHELL_NAME=$(basename "$SHELL")
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

        # Add to PATH in shell config
        if [ -f "$SHELL_RC" ] && ! grep -q "$INSTALL_DIR" "$SHELL_RC"; then
            echo "" >> "$SHELL_RC"
            echo "# uvup path" >> "$SHELL_RC"
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
            echo "Added to PATH in $SHELL_RC"
        fi

        # Add to current session
        export PATH="$INSTALL_DIR:$PATH"
        PATH_CONFIGURED=1
        ;;
esac

echo ""
echo "Configuring shell integration..."

# Run uvup init to configure all shells
if command -v uvup >/dev/null 2>&1; then
    uvup init
    echo ""
    echo "Shell integration configured successfully!"
    echo ""
    echo "To start using uvup, restart your terminal or run:"

    SHELL_NAME=$(basename "$SHELL")
    case "$SHELL_NAME" in
        bash)
            echo "  source ~/.bashrc"
            ;;
        zsh)
            echo "  source ~/.zshrc"
            ;;
        fish)
            echo "  source ~/.config/fish/config.fish"
            ;;
        *)
            echo "  source ~/.profile"
            ;;
    esac
else
    echo "Warning: uvup not found in PATH"
    echo "Please add $INSTALL_DIR to your PATH and run: uvup init"
fi

echo ""
