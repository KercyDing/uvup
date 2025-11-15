# uvup

A conda-like environment manager for [uv](https://github.com/astral-sh/uv).

## Vision

uvup aims to be a companion tool for uv, providing a familiar conda-like interface for centralized Python virtual environment management.

## Core Philosophy

- **Enhancement, not replacement**: uvup calls uv for actual work
- **User experience first**: Familiar commands and seamless activation
- **Best practices**: Following proven patterns from conda and rustup
- **Lightweight and cross-platform**: Single binary, works everywhere

## Installation

### For Users

Coming soon - prebuilt binaries will be available on GitHub Releases.

### For Developers

1. Clone the repository:
```bash
git clone https://github.com/KercyDing/uvup.git
cd uvup
```

2. Install from source:
```bash
cargo install --path .
```

3. Initialize shell integration:
```bash
# Add to your shell configuration
echo 'eval "$(uvup init)"' >> ~/.zshrc  # or ~/.bashrc for bash
source ~/.zshrc
```

#### Development Workflow

During development, you have several options:

**Option 1: Reinstall after changes (Recommended)**
```bash
# After modifying code
cargo install --path .
```

**Option 2: Use cargo run for quick testing**
```bash
cargo run -- --help
cargo run -- create test-env
cargo run -- list
```

**Option 3: Build and test manually**
```bash
cargo build
./target/debug/uvup --help
```

## Planned Features

### MVP (v0.1.0) - ✅ Completed

- ✅ `uvup init` - Shell integration
- ✅ `uvup create <name>` - Create environments
- ✅ `uvup activate <name>` - Activate environments (via shell hook)
- ✅ `uvup list` - List all environments
- ✅ `uvup remove <name>` - Remove environments

### Future Versions

- `uvup default <name>` - Set default environment (auto-activate on new terminal)
- `uvup undefault` - Remove default environment
- Enhanced shell support (Fish, PowerShell)
- Cross-platform distribution (Homebrew, Scoop, Winget)

## Usage

### Quick Start

```bash
# Create a new environment
uvup create myproject

# Create with specific Python version
uvup create myproject --python 3.11

# List all environments
uvup list

# Activate an environment
uvup activate myproject

# Install packages (using uv)
uv pip install numpy pandas

# Deactivate
uvup deactivate

# Remove an environment
uvup remove myproject
```

## Scope

uvup focuses on **environment management only**. For package management, use uv directly:

```bash
# Environment management with uvup
uvup create myproject
uvup activate myproject

# Package management with uv
uv pip install numpy pandas
```

## License

[MIT](LICENSE)
