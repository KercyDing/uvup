# Command Reference

Complete reference for all uvup commands.

## Table of Contents

- [Environment Management](#environment-management)
  - [create](#create)
  - [list](#list)
  - [remove](#remove)
- [Environment Operations](#environment-operations)
  - [clone](#clone)
  - [new](#new)
  - [sync](#sync)
- [System](#system)
  - [init](#init)
  - [update](#update)

## Environment Management

### create

Create a new empty virtual environment.

**Usage:**
```bash
uvup create <name> [OPTIONS]
```

**Arguments:**
- `<name>` - Name of the environment to create

**Options:**
- `-p, --python <version>` - Python version (default: 3.12)

**Examples:**
```bash
# Create environment with default Python
uvup create myproject

# Create with specific Python version
uvup create myproject --python 3.11
uvup create --python 3.11 myproject
```

**Notes:**
- Creates an empty pyproject.toml with minimal configuration
- Initializes a virtual environment with uv
- Environment is created in `~/.uvup/<name>/`

---

### list

List all available environments.

**Usage:**
```bash
uvup list
```

**Output:**
- Lists all environments in `~/.uvup/`
- Shows "No environments found." if empty

**Examples:**
```bash
uvup list
```

---

### remove

Remove an existing environment.

**Usage:**
```bash
uvup remove <name>
```

**Arguments:**
- `<name>` - Name of the environment to remove

**Examples:**
```bash
uvup remove myproject
```

**Notes:**
- Permanently deletes the environment directory
- Cannot be undone
- Fails if environment doesn't exist
- Only one environment can be removed at a time

---

## Environment Operations

### clone

Clone an existing environment to create an exact 1:1 copy.

**Usage:**
```bash
uvup clone <source> <target>
```

**Arguments:**
- `<source>` - Source environment name
- `<target>` - Target environment name

**Examples:**
```bash
# Create exact backup
uvup clone myproject myproject-backup

# Clone for testing
uvup clone production testing
```

**What Gets Cloned:**
- ✅ `pyproject.toml` - Project configuration
- ✅ `hello.py` - Demo file (if exists)
- ✅ `uv.lock` - Lock file (if exists)
- ✅ Virtual environment - Fresh venv with same packages

**What Doesn't Get Cloned:**
- ❌ `.venv/` directory (recreated fresh)
- ❌ Custom files (only standard files)

**Notes:**
- Pure 1:1 copy with **no modification options**
- Use `new` if you need to modify during copy
- Automatically syncs packages from lock file

---

### new

Create a new project from a template environment with modification support.

**Usage:**
```bash
uvup new <name> --template <template> [OPTIONS]
```

**Arguments:**
- `<name>` - Project name
- `--template <template>` - Template environment name

**Options:**
- `-p, --python <version>` - Override Python version
- `--exclude <packages>` - Exclude packages (comma-separated)
- `--include <packages>` - Include only these packages (comma-separated)
- `--path <directory>` - Create in custom directory (default: current dir)
- `--dry-run` - Preview changes without creating

**Examples:**
```bash
# Basic project creation
uvup new myapp --template web-template

# Custom Python version
uvup new myapp --template web-template --python 3.11

# Exclude development tools
uvup new myapp --template web-template --exclude pytest,black,mypy

# Include only specific packages
uvup new myapp --template web-template --include numpy,pandas,requests

# Create in specific directory
uvup new myapp --template web-template --path ~/projects

# Preview without creating
uvup new myapp --template web-template --exclude pytest --dry-run
```

**Modification Behavior:**

1. **Project Name**: Automatically updated in pyproject.toml
2. **Python Version**: Overridden if `--python` specified
3. **Dependencies**: Filtered by `--exclude` or `--include`
4. **Optional Dependencies**: Also filtered, empty groups removed

**Filtering Rules:**

- `--exclude`: Removes specified packages from both main and optional dependencies
- `--include`: Keeps ONLY specified packages (removes all others)
- Cannot use both `--exclude` and `--include` together
- Package names are case-insensitive
- Handles PEP 508 specifiers (e.g., `requests[http3]>=2.0`)

**Dry-run Output:**
```
-- Dry Run Mode --

Template: 'web-template' (Python 3.12)
Project:  'myapp' (Python 3.11)
Location: /Users/you/myapp

Python version change:
  3.12 → 3.11

Filters applied:
  Exclude: pytest, black

Dependency changes:
  Removed (2):
    - pytest>=7.0.0
    - black>=23.0.0
  Kept (5):

Optional dependencies:
  [dev]: Removed (group is empty after filtering)
  [viz]: No changes

To create this project, run the same command without --dry-run
```

**Notes:**
- Creates project in `<path>/<name>/` (default: `./<name>/`)
- Fails if target directory already exists
- Automatically runs `uv lock` and `uv sync`
- Project is ready to use immediately

---

### sync

Sync current project with a template environment.

**Usage:**
```bash
uvup sync --template <template> [OPTIONS]
```

**Arguments:**
- `--template <template>` - Template environment name

**Options:**
- `-p, --python <version>` - Override Python version
- `--exclude <packages>` - Exclude packages (comma-separated)
- `--include <packages>` - Include only these packages (comma-separated)
- `--dry-run` - Preview changes without syncing

**Examples:**
```bash
# Sync with template
cd myproject
uvup sync --template web-template

# Sync with Python version change
uvup sync --template web-template --python 3.11

# Sync without dev dependencies
uvup sync --template web-template --exclude pytest,black,mypy

# Sync only core packages
uvup sync --template web-template --include numpy,pandas,requests

# Preview changes
uvup sync --template web-template --dry-run
```

**Sync Behavior:**

1. **Dependencies**: Replaced with template's dependencies (filtered)
2. **Optional Dependencies**: Replaced with template's optional-dependencies (filtered)
3. **Python Version**: Updated if `--python` specified
4. **Project Name**: Preserved (NOT changed)

**Safety Features:**

- ✅ Automatic backup: `pyproject.toml.backup` created before changes
- ✅ Rollback on error: Backup restored if `uv lock` or `uv sync` fails
- ✅ Backup cleanup: Removed on successful completion

**Dry-run Output:**
```
-- Dry Run Mode --

Template: 'web-template' (Python 3.12)
Current:  /Users/you/myproject (Python 3.12)

Dependency changes:
  Added (2):
    + fastapi>=0.100.0
    + uvicorn>=0.23.0
  Removed (1):
    - flask>=2.3.0
  Kept (3):

Optional dependencies:
  [dev]: Modified (5 packages)
  [viz]: No changes

To sync this project, run the same command without --dry-run
```

**Notes:**
- Must be run from project directory containing `pyproject.toml`
- Fails if no `pyproject.toml` found
- Changes take effect immediately (except in dry-run mode)
- Backup file is left if user interrupts (Ctrl+C)

---

## System

### init

Initialize shell integration for uvup.

**Usage:**
```bash
uvup init
```

**Supported Shells:**
- Bash
- Zsh
- Fish
- PowerShell

**What It Does:**
- Detects your current shell
- Adds uvup initialization to shell config
- Enables `uvup activate` and `uvup deactivate` commands

**Examples:**
```bash
# Initialize for current shell
uvup init
```

**Shell Config Files:**
- Bash: `~/.bashrc`
- Zsh: `~/.zshrc`
- Fish: `~/.config/fish/config.fish`
- PowerShell: `$PROFILE`

**Notes:**
- Run this once after installation
- Restart shell or source config file to apply
- Safe to run multiple times (idempotent)

---

### update

Update uvup to the latest version.

**Usage:**
```bash
uvup update [OPTIONS]
```

**Options:**
- `-c, --check` - Only check for updates without installing

**Examples:**
```bash
# Update to latest version
uvup update

# Check if update available
uvup update --check
```

**Notes:**
- Downloads latest release from GitHub
- Replaces current binary in-place
- Preserves all environments and configuration

---

## Shell-only Commands

These commands are available only after running `uvup init`:

### activate

Activate a virtual environment.

**Usage:**
```bash
uvup activate <name>
```

**Arguments:**
- `<name>` - Environment name to activate

**Examples:**
```bash
uvup activate myproject
```

### deactivate

Deactivate the current virtual environment.

**Usage:**
```bash
uvup deactivate
```

**Examples:**
```bash
uvup deactivate
```

---

## Command Decision Tree

**Need to create something?**
- Empty environment → `create`
- Exact copy → `clone`
- New project from template → `new`

**Need to update?**
- Current project from template → `sync`
- uvup itself → `update`

**Need to manage?**
- See all environments → `list`
- Delete environment → `remove`

**Need to use?**
- Enable activation → `init`
- Enter environment → `activate`
- Exit environment → `deactivate`
