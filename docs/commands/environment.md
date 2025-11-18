# Venv

Commands for creating, listing, and deleting virtual environments.

## create

Create a new empty virtual environment.

### Usage

```bash
uvup create <name> [OPTIONS]
```

### Arguments

- `<name>` - Name of the environment to create

### Options

- `-p, --python <version>` - Python version (default: 3.12)

### Examples

```bash
# Create environment with default Python
uvup create myproject

# Create with specific Python version
uvup create myproject --python 3.11
uvup create --python 3.11 myproject
```

### Notes

- Creates an empty pyproject.toml with minimal configuration
- Initializes a virtual environment with uv
- Environment is created in `~/.uvup/<name>/`

---

## list

List all available environments.

### Usage

```bash
uvup list
```

### Output

- Lists all environments in `~/.uvup/`
- Shows "No environments found." if empty

### Examples

```bash
uvup list
```

---

## delete

Delete an existing environment.

### Usage

```bash
uvup delete <name>
```

### Arguments

- `<name>` - Name of the environment to delete

### Examples

```bash
uvup delete myproject
```

### Notes

- Permanently deletes the environment directory
- Cannot be undone
- Fails if environment doesn't exist
- Only one environment can be deleted at a time

---

## clone

Clone an existing environment to create an exact 1:1 copy.

### Usage

```bash
uvup clone <source> <target>
```

### Arguments

- `<source>` - Source environment name
- `<target>` - Target environment name

### Examples

```bash
# Create exact backup
uvup clone myproject myproject-backup

# Clone for testing
uvup clone production testing
```

### What Gets Cloned

- `pyproject.toml` - Project configuration
- `hello.py` - Demo file (if exists)
- `uv.lock` - Lock file (if exists)
- Virtual environment - Fresh venv with same packages

### What Doesn't Get Cloned

- `.venv/` directory (recreated fresh)
- Custom files (only standard files)

### Notes

- Pure 1:1 copy with **no modification options**
- Use `new` if you need to modify during copy
- Automatically syncs packages from lock file
