# Development Workflows

Daily development patterns with uvup.

## Creating Template Environments

Create reusable templates for different project types:

```bash
# 1. Create base web template
uvup create web-template
uvup activate web-template
uvup add fastapi uvicorn pydantic
uvup add --group dev pytest black mypy ruff
uvup deactivate

# 2. Create data science template
uvup create ds-template
uvup activate ds-template
uvup add numpy pandas matplotlib seaborn scikit-learn
uvup add --group dev jupyter ipykernel pytest
uvup deactivate

# 3. Create CLI tool template
uvup create cli-template
uvup activate cli-template
uvup add click rich typer
uvup add --group dev pytest pytest-cov
uvup deactivate
```

## Starting New Projects from Templates

### Basic Project Creation

```bash
# Start a new web API project
uvup new my-api --template web-template

# Start without dev tools (for production containers)
uvup new my-api --template web-template --exclude pytest,black,mypy,ruff

# Start with only core dependencies
uvup new minimal-api --template web-template --include fastapi,uvicorn,pydantic
```

### Preview Before Creating

Always use `--dry-run` to preview changes:

```bash
uvup new my-api --template web-template --exclude pytest --dry-run
```

## Syncing Existing Projects

### Basic Sync

Update project with latest template changes:

```bash
cd my-existing-project
uvup sync --template web-template --dry-run
# Review changes...
uvup sync --template web-template
```

### Selective Sync

Sync but keep dev tools separate:

```bash
uvup sync --template web-template --exclude pytest,black,mypy
```

## Managing Multiple Projects

```bash
# List all projects
uvup list

# Create project-specific environments
uvup new client-a-api --template web-template
uvup new client-b-api --template web-template
uvup new data-pipeline --template ds-template

# Switch between projects
cd ~/projects/client-a-api
uvup activate client-a-api
# ... work ...
uvup deactivate
```

## Experimenting Safely

### Clone Before Experimenting

```bash
# Clone production environment for testing
uvup clone production-env experiment-env

# Activate and test new packages
uvup activate experiment-env
uvup add experimental-package
python -m pytest
uvup deactivate

# If successful, update production
uvup activate production-env
uvup add experimental-package
uvup deactivate

# If failed, just remove experiment
uvup delete experiment-env
```

## Upgrading Dependencies

### Safe Upgrade Strategy

```bash
# 1. Create backup before upgrade
uvup clone my-project my-project-backup

# 2. Upgrade in original
uvup activate my-project
uvup lock --upgrade
uv sync
python -m pytest
uvup deactivate

# 3. If successful, clean up backup
uvup delete my-project-backup

# 4. If failed, restore from backup
uvup delete my-project
uvup clone my-project-backup my-project
uvup delete my-project-backup
```

## Best Practices

### Template Naming Conventions

```bash
# Use descriptive names
uvup create web-api-template      # Good
uvup create template1             # Bad

# Use versioning for templates
uvup create web-api-v1-template
uvup create web-api-v2-template

# Use prefixes for categories
uvup create ml-pytorch-template
uvup create ml-tensorflow-template
uvup create web-fastapi-template
uvup create web-flask-template
```

### Regular Updates

Keep your templates fresh:

```bash
# Monthly template maintenance
uvup activate web-template
uvup lock --upgrade
uv sync
# Test template...
uvup deactivate
```
