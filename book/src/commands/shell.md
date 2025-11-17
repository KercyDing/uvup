# Shell Commands

Shell integration commands available after running `uvup init`.

## activate

Activate a virtual environment.

### Usage

```bash
uvup activate <name>
```

### Arguments

- `<name>` - Environment name to activate

### Examples

```bash
uvup activate myproject
```

### What It Does

1. Checks if the environment exists
2. Activates the `.venv` within the environment
3. Sets `UVUP_ACTIVE_ENV` environment variable
4. Modifies shell prompt to show environment name
5. Enables package management commands

### Effects

After activation:

- Shell prompt shows `(myproject)`
- `python` points to environment's Python
- `pip` and other tools use environment's packages
- Package commands (`add`, `remove`, `lock`, `tree`) are enabled
- Works from any directory

### Notes

- Requires `uvup init` to be set up first
- Only one environment can be active at a time
- Activating a new environment automatically deactivates the current one

---

## deactivate

Deactivate the current virtual environment.

### Usage

```bash
uvup deactivate
```

### Examples

```bash
uvup deactivate
```

### What It Does

1. Deactivates the current virtual environment
2. Clears `UVUP_ACTIVE_ENV` environment variable
3. Restores original shell prompt
4. Restores original PATH
5. Disables package management commands

### Effects

After deactivation:

- Shell prompt returns to normal
- `python` points to system Python
- Package commands will fail with "No active environment" error

### Notes

- Requires `uvup init` to be set up first
- Safe to run even if no environment is active
- Does not delete or modify the environment
