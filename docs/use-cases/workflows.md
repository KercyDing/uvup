# Workflows

Common workflows and usage patterns for daily development.

## Quick Project Creation

### Data Science Project

```bash
uvup create data-project
uvup activate data-project
uvup add numpy pandas matplotlib jupyter scikit-learn
```

### Web API Project

```bash
uvup create api-project
uvup activate api-project
uvup add fastapi uvicorn pydantic sqlalchemy
uvup add --group dev pytest black mypy
```

### CLI Tool Project

```bash
uvup create cli-tool
uvup activate cli-tool
uvup add click rich typer
uvup add --group dev pytest
```

## Working with Templates

### Create a Template

Create reusable environment configurations:

```bash
# Create web template
uvup create web-template
uvup activate web-template
uvup add fastapi uvicorn sqlalchemy pydantic
uvup add --group dev pytest black mypy ruff
```

### Start New Project from Template

```bash
# Create new project from template
uvup new myapi --template web-template

# With custom Python version
uvup new myapi --template web-template --python 3.11

# Exclude dev tools
uvup new myapi --template web-template --exclude pytest,black,mypy
```

### Preview Before Creating

```bash
# See what would be created
uvup new myapi --template web-template --dry-run
```

## Syncing Existing Projects

Update existing projects from templates:

```bash
# Basic sync
cd myproject
uvup sync --template web-template

# Preview changes first
uvup sync --template web-template --dry-run

# Selective sync (exclude certain packages)
uvup sync --template web-template --exclude pytest,black
```

## Managing Multiple Projects

```bash
# List all environments
uvup list

# Switch between projects
uvup activate project-a
# ... work on project-a ...
uvup deactivate

uvup activate project-b
# ... work on project-b ...
uvup deactivate
```

## Experimenting Safely

Clone environments before making changes:

```bash
# Clone production environment
uvup clone production-env experiment-env

# Activate and test
uvup activate experiment-env
uvup add experimental-package
uv run python -m pytest       # Classic usage
uv run python -m pytest # uv-like usage
uvup deactivate

# If successful, update production
uvup activate production-env
uvup add experimental-package
uvup deactivate

# If failed, delete experiment
uvup delete experiment-env
```

## Upgrading Dependencies

Safe upgrade strategy:

```bash
# 1. Create backup
uvup clone my-project my-project-backup

# 2. Upgrade in original
uvup activate my-project
uvup lock --upgrade
uv sync
uv run python -m pytest
uvup deactivate

# 3. If successful, clean up backup
uvup delete my-project-backup

# 4. If failed, restore from backup
uvup delete my-project
uvup clone my-project-backup my-project
uvup delete my-project-backup
```

## Team Collaboration

### Share Environment Definitions

Create a team template:

```bash
# On one team member's machine
uvup create team-template
uvup activate team-template
uvup add requests fastapi sqlalchemy pydantic
uvup add --group dev pytest black mypy
uvup deactivate

# Share pyproject.toml and uv.lock
git add pyproject.toml uv.lock
git commit -m "Add team environment template"
git push
```

### Team Members Setup

```bash
# Other team members
git pull
uvup create my-project
cd my-project
cp ../team-template/pyproject.toml .
cp ../team-template/uv.lock .
uv sync
```

### Code Review Environments

```bash
# Reviewer creates temporary environment
uvup clone main-env pr-123-review
uvup activate pr-123-review

# Apply PR changes
git checkout pr-branch

# Sync dependencies if changed
uvup sync --template main-template

# Test the changes
uv run python -m pytest      # Classic usage
uv run python -m pytest # uv-like usage
uvup deactivate

# Clean up
uvup delete pr-123-review
```

## Best Practices

### Template Naming

```bash
# Use descriptive names
uvup create web-template        # Good
uvup create ml-template         # Good
uvup create template1           # Bad
```

### Regular Updates

```bash
# Update templates regularly
uvup activate web-template
uvup lock --upgrade
uv sync

# Sync projects from updated template
cd myproject
uvup sync --template web-template
```

### Lock File Strategy

Always commit `uv.lock` for reproducible environments:

```bash
git add pyproject.toml uv.lock
git commit -m "Update dependencies"
```
