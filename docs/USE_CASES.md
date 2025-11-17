# Use Cases

Real-world scenarios and workflows with uvup.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Workflows](#development-workflows)
- [Team Collaboration](#team-collaboration)
- [Project Management](#project-management)
- [Advanced Patterns](#advanced-patterns)

## Getting Started

### First Time Setup

```bash
# 1. Install uvup
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh

# 2. Initialize shell integration
uvup init

# 3. Restart your shell or source config
source ~/.bashrc  # or ~/.zshrc, etc.

# 4. Create your first environment
uvup create my-first-project --python 3.12

# 5. Activate and use it
uvup activate my-first-project
uvup add requests numpy
python -c "import requests; print(requests.__version__)"
uvup deactivate
```

### Quick Project Creation

```bash
# Create a simple data science project
uvup create ds-project
uvup activate ds-project
uvup add numpy pandas matplotlib jupyter
jupyter notebook
```

## Development Workflows

### Creating Template Environments

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

### Starting New Projects from Templates

```bash
# Start a new web API project
uvup new my-api --template web-template

# Start without dev tools (for production containers)
uvup new my-api --template web-template --exclude pytest,black,mypy,ruff

# Start with only core dependencies
uvup new minimal-api --template web-template --include fastapi,uvicorn,pydantic
```

### Syncing Existing Projects

```bash
# Update project with latest template changes
cd my-existing-project
uvup sync --template web-template --dry-run
# Review changes...
uvup sync --template web-template

# Sync but keep dev tools separate
uvup sync --template web-template --exclude pytest,black,mypy
```

## Team Collaboration

### Sharing Environment Definitions

**Scenario**: Team wants consistent development environments.

```bash
# Team lead creates template
uvup create team-base-template --python 3.11
uvup activate team-base-template
uvup add requests httpx pydantic sqlalchemy
uvup add --group dev pytest black mypy ruff pre-commit
uvup add --group docs sphinx sphinx-rtd-theme
uvup deactivate

# Team lead shares the pyproject.toml
cp ~/.uvup/team-base-template/pyproject.toml ./templates/

# Team members create from template
uvup create team-base-template
uvup activate team-base-template
# Copy pyproject.toml to environment
cp ./templates/pyproject.toml ~/.uvup/team-base-template/
cd ~/.uvup/team-base-template
uv lock
uv sync
uvup deactivate

# Now everyone can create consistent projects
uvup new my-feature --template team-base-template
```

### Onboarding New Developers

```bash
# New developer setup script
#!/bin/bash

# Install uvup
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
uvup init

# Create standard templates
uvup create backend-template
cp templates/backend.toml ~/.uvup/backend-template/pyproject.toml
cd ~/.uvup/backend-template && uv sync && cd -

uvup create frontend-template
cp templates/frontend.toml ~/.uvup/frontend-template/pyproject.toml
cd ~/.uvup/frontend-template && uv sync && cd -

# Clone current project
git clone https://github.com/company/project
cd project
uvup sync --template backend-template
```

## Project Management

### Managing Multiple Projects

```bash
# List all projects
uvup list

# Create project-specific environments
uvup new client-a-api --template web-template
uvup new client-b-api --template web-template
uvup new data-pipeline --template ds-template

# Switch between projects
cd ~/projects/client-a-api && uvup activate client-a-api
# ... work on client A ...
uvup deactivate

cd ~/projects/client-b-api && uvup activate client-b-api
# ... work on client B ...
uvup deactivate
```

### Experimenting Safely

```bash
# Clone production environment for testing
uvup clone production-env experiment-env

# Activate and test new packages
uvup activate experiment-env
uvup add experimental-package
python -m pytest
# ... experiment ...
uvup deactivate

# If successful, update template
uvup sync --template production-template --dry-run
# Review and apply
uvup sync --template production-template

# If failed, just remove experiment
uvup delete experiment-env
```

### Upgrading Dependencies

```bash
# Create backup before upgrade
uvup clone my-project my-project-backup

# Upgrade in original
uvup activate my-project
uvup add numpy@latest pandas@latest
python -m pytest
# ... verify everything works ...
uvup deactivate

# If successful, update template
cd ~/.uvup/my-project-template
uvup add numpy@latest pandas@latest
uv lock

# If failed, restore from backup
uvup delete my-project
uvup clone my-project-backup my-project
uvup delete my-project-backup
```

## Advanced Patterns

### Multi-Stage Project Setup

**Scenario**: Different dependency sets for dev/test/prod.

```bash
# Base template with core dependencies
uvup create base-template
uvup activate base-template
uvup add fastapi uvicorn pydantic sqlalchemy
uvup deactivate

# Development environment (full stack)
uvup new my-project-dev --template base-template
cd my-project-dev
uvup add --group dev pytest black mypy ruff ipython
uvup add --group dev pytest-cov pytest-asyncio

# Testing environment (dev + test tools only)
uvup new my-project-test --template base-template
cd ../my-project-test
uvup add --group dev pytest pytest-cov pytest-asyncio

# Production environment (minimal)
uvup new my-project-prod --template base-template \
  --exclude pytest,black,mypy,ruff,ipython
```

### Microservices Development

**Scenario**: Multiple services with shared base, different extras.

```bash
# Create shared base template
uvup create microservice-base
uvup activate microservice-base
uvup add fastapi uvicorn pydantic httpx
uvup add --group dev pytest pytest-asyncio
uvup deactivate

# Create service-specific projects
uvup new auth-service --template microservice-base
cd auth-service
uvup add pyjwt passlib[bcrypt] python-multipart

uvup new user-service --template microservice-base
cd ../user-service
uvup add sqlalchemy asyncpg alembic

uvup new notification-service --template microservice-base
cd ../notification-service
uvup add aiosmtplib jinja2 celery
```

### Educational Environments

**Scenario**: Teaching Python with consistent student environments.

```bash
# Instructor creates course template
uvup create course-python-basics
uvup activate course-python-basics
uvup add ipython jupyter notebook pytest
uvup deactivate

# Students clone for each module
uvup new module-01-basics --template course-python-basics
uvup new module-02-functions --template course-python-basics
uvup new module-03-classes --template course-python-basics

# Each module can have specific packages
cd module-02-functions
uvup add hypothesis  # for property-based testing

cd ../module-03-classes
uvup add attrs pydantic  # for class examples
```

### CI/CD Integration

**Scenario**: Reproducible CI environments.

```bash
# Local development
uvup create ci-template
uvup activate ci-template
uvup add pytest pytest-cov black mypy ruff
uvup deactivate

# Export for CI (Dockerfile)
cat > Dockerfile << 'DOCKERFILE'
FROM python:3.12-slim

# Install uv
RUN pip install uv

# Copy project
COPY . /app
WORKDIR /app

# Install dependencies
RUN uv sync

# Run tests
CMD ["uv", "run", "pytest"]
DOCKERFILE

# Or use template in CI script
# .github/workflows/test.yml
cat > .github/workflows/test.yml << 'YAML'
name: Test
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.12'
      - name: Install uvup
        run: curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
      - name: Create from template
        run: |
          uvup create ci-env --python 3.12
          cp pyproject.toml ~/.uvup/ci-env/
          cd ~/.uvup/ci-env && uv sync
      - name: Run tests
        run: cd ~/.uvup/ci-env && uv run pytest
YAML
```

### Dependency Isolation Testing

**Scenario**: Test with different dependency versions.

```bash
# Base template
uvup create app-template
uvup activate app-template
uvup add requests flask numpy
uvup deactivate

# Test with minimum versions
uvup new app-min-versions --template app-template
cd app-min-versions
# Manually set minimum versions in pyproject.toml
uv lock
uv sync
uv run pytest

# Test with latest versions
uvup new app-latest-versions --template app-template
cd ../app-latest-versions
uvup add requests@latest flask@latest numpy@latest
uv run pytest

# Test without optional dependencies
uvup new app-no-optional --template app-template \
  --exclude dev-package-1,dev-package-2
cd ../app-no-optional
uv run pytest
```

## Tips and Best Practices

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

### Regular Maintenance

```bash
#!/bin/bash
# monthly-maintenance.sh

# Update uvup itself
uvup update

# List all environments
echo "Current environments:"
uvup list

# Update templates with latest packages
for template in web-template ds-template cli-template; do
  if uvup list | grep -q "$template"; then
    echo "Updating $template..."
    cd ~/.uvup/$template
    uvup add --upgrade $(uv pip list --format=freeze | cut -d= -f1)
    uv lock
  fi
done
```

### Backup Strategy

```bash
#!/bin/bash
# backup-environments.sh

BACKUP_DIR=~/uvup-backups/$(date +%Y%m%d)
mkdir -p "$BACKUP_DIR"

# Backup all pyproject.toml files
for env in ~/.uvup/*/; do
  name=$(basename "$env")
  cp "$env/pyproject.toml" "$BACKUP_DIR/$name.toml"
done

echo "Backed up to $BACKUP_DIR"
```

### Cleanup Unused Environments

```bash
#!/bin/bash
# cleanup-old-environments.sh

# List all environments with creation time
for env in ~/.uvup/*/; do
  name=$(basename "$env")
  age=$(find "$env" -maxdepth 0 -mtime +30)
  if [ -n "$age" ]; then
    echo "Old environment: $name (> 30 days)"
    read -p "Remove? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      uvup remove "$name"
    fi
  fi
done
```
