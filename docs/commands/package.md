# Package

Manage packages in activated environments. All commands require an activated environment and work from any directory.

## add

Add packages to the active environment.

### Usage

```bash
uvup add <packages...> [OPTIONS]
```

### Arguments

- `<packages...>` - One or more packages to add

### Options

- `--group <name>` - Add to optional dependency group

### Examples

```bash
# Activate environment first
uvup activate myproject

# Add packages
uvup add requests numpy pandas

# Add with version specifiers
uvup add "requests>=2.28.0" "numpy<2.0"

# Add to development group
uvup add --group dev pytest black mypy
```

### Notes

- Requires an activated environment
- Updates `pyproject.toml` and `uv.lock`
- Installs packages immediately
- Works from any directory (not just project root)

---

## remove

Remove packages from the active environment.

### Usage

```bash
uvup remove <packages...> [OPTIONS]
```

### Arguments

- `<packages...>` - One or more packages to remove

### Options

- `--group <name>` - Remove from optional dependency group

### Examples

```bash
# Activate environment first
uvup activate myproject

# Remove packages
uvup remove requests numpy

# Remove from development group
uvup remove --group dev pytest
```

### Notes

- Requires an activated environment
- Updates `pyproject.toml` and `uv.lock`
- Uninstalls packages immediately
- Works from any directory (not just project root)

---

## lock

Update the lockfile of the active environment.

### Usage

```bash
uvup lock [OPTIONS]
```

### Options

- `--upgrade` - Upgrade all packages to their latest compatible versions

### Examples

```bash
# Activate environment first
uvup activate myproject

# Update lock file
uvup lock

# Upgrade all packages
uvup lock --upgrade
```

### Notes

- Requires an activated environment
- Updates `uv.lock` based on `pyproject.toml`
- Does not install packages (use `uv sync` to install)
- Works from any directory (not just project root)

---

## tree

Display the dependency tree of the active environment.

### Usage

```bash
uvup tree [OPTIONS]
```

### Options

- `--depth <n>` - Maximum depth to display

### Examples

```bash
# Activate environment first
uvup activate myproject

# Show full dependency tree
uvup tree

# Limit depth
uvup tree --depth 2
```

### Notes

- Requires an activated environment
- Shows hierarchical view of dependencies
- Helps identify dependency conflicts
- Works from any directory (not just project root)
