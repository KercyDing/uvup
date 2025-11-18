# Project

Template-based project creation and synchronization.

## new

Create a new project from a template environment with modification support.

### Usage

```bash
uvup new <name> --template <template> [OPTIONS]
```

### Arguments

- `<name>` - Project name
- `--template <template>` - Template environment name

### Options

- `-p, --python <version>` - Override Python version
- `--exclude <packages>` - Exclude packages (comma-separated)
- `--include <packages>` - Include only these packages (comma-separated)
- `--path <directory>` - Create in custom directory (default: current dir)
- `--dry-run` - Preview changes without creating

### Examples

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

### Modification Behavior

1. **Project Name**: Automatically updated in pyproject.toml
2. **Python Version**: Overridden if `--python` specified
3. **Dependencies**: Filtered by `--exclude` or `--include`
4. **Optional Dependencies**: Also filtered, empty groups removed

### Filtering Rules

- `--exclude`: Removes specified packages from both main and optional dependencies
- `--include`: Keeps ONLY specified packages (removes all others)
- Cannot use both `--exclude` and `--include` together
- Package names are case-insensitive
- Handles PEP 508 specifiers (e.g., `requests[http3]>=2.0`)

### Dry-run Output

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

### Notes

- Creates project in `<path>/<name>/` (default: `./<name>/`)
- Fails if target directory already exists
- Automatically runs `uv lock` and `uv sync`
- Project is ready to use immediately

---

## sync

Sync current project with a template environment.

### Usage

```bash
uvup sync --template <template> [OPTIONS]
```

### Arguments

- `--template <template>` - Template environment name

### Options

- `-p, --python <version>` - Override Python version
- `--exclude <packages>` - Exclude packages (comma-separated)
- `--include <packages>` - Include only these packages (comma-separated)
- `--dry-run` - Preview changes without syncing

### Examples

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

### Sync Behavior

1. **Dependencies**: Replaced with template's dependencies (filtered)
2. **Optional Dependencies**: Replaced with template's optional-dependencies (filtered)
3. **Python Version**: Updated if `--python` specified
4. **Project Name**: Preserved (NOT changed)

### Safety Features

- Automatic backup: `pyproject.toml.backup` created before changes
- Rollback on error: Backup restored if `uv lock` or `uv sync` fails
- Backup cleanup: Removed on successful completion

### Dry-run Output

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

### Notes

- Must be run from project directory containing `pyproject.toml`
- Fails if no `pyproject.toml` found
- Changes take effect immediately (except in dry-run mode)
- Backup file is left if user interrupts (Ctrl+C)
