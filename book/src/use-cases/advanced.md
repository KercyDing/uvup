# Advanced Patterns

Complex workflows and advanced usage scenarios.

## Multi-Stage Project Setup

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
uvup activate my-project-dev
uvup add --group dev pytest black mypy ruff ipython
uvup add --group dev pytest-cov pytest-asyncio
uvup deactivate

# Testing environment (dev + test tools only)
uvup new my-project-test --template base-template
cd ../my-project-test
uvup activate my-project-test
uvup add --group dev pytest pytest-cov pytest-asyncio
uvup deactivate

# Production environment (minimal)
uvup new my-project-prod --template base-template \
  --exclude pytest,black,mypy,ruff,ipython
```

## Microservices Development

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
uvup activate auth-service
uvup add pyjwt passlib python-multipart
uvup deactivate

uvup new user-service --template microservice-base
cd ../user-service
uvup activate user-service
uvup add sqlalchemy asyncpg alembic
uvup deactivate

uvup new notification-service --template microservice-base
cd ../notification-service
uvup activate notification-service
uvup add aiosmtplib jinja2 celery
uvup deactivate
```

## Educational Environments

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
uvup activate module-02-functions
uvup add hypothesis  # for property-based testing
uvup deactivate

cd ../module-03-classes
uvup activate module-03-classes
uvup add attrs pydantic  # for class examples
uvup deactivate
```

## CI/CD Integration

### Docker Integration

```dockerfile
FROM python:3.12-slim

# Install uv
RUN pip install uv

# Copy project
COPY . /app
WORKDIR /app

# Install dependencies from pyproject.toml
RUN uv sync

# Run tests
CMD ["uv", "run", "pytest"]
```

### GitHub Actions

```yaml
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
      
      - name: Install uv
        run: pip install uv
      
      - name: Install dependencies
        run: uv sync
      
      - name: Run tests
        run: uv run pytest
```

## Dependency Isolation Testing

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
# Edit pyproject.toml to set minimum versions
uvup activate app-min-versions
uvup lock
uv sync
uv run pytest
uvup deactivate

# Test with latest versions
uvup new app-latest-versions --template app-template
cd ../app-latest-versions
uvup activate app-latest-versions
uvup lock --upgrade
uv sync
uv run pytest
uvup deactivate
```

## Backup Strategies

### Automated Backup Script

```bash
#!/bin/bash
# backup-environments.sh

BACKUP_DIR=~/uvup-backups/$(date +%Y%m%d)
mkdir -p "$BACKUP_DIR"

# Backup all pyproject.toml files
for env in ~/.uvup/*/; do
  name=$(basename "$env")
  if [ -f "$env/pyproject.toml" ]; then
    cp "$env/pyproject.toml" "$BACKUP_DIR/$name.toml"
    echo "Backed up $name"
  fi
done

echo "Backups saved to $BACKUP_DIR"
```

### Restore from Backup

```bash
#!/bin/bash
# restore-environment.sh

if [ -z "$1" ] || [ -z "$2" ]; then
  echo "Usage: $0 <backup-file> <env-name>"
  exit 1
fi

BACKUP_FILE="$1"
ENV_NAME="$2"

# Create environment if it doesn't exist
if ! uvup list | grep -q "$ENV_NAME"; then
  uvup create "$ENV_NAME"
fi

# Restore pyproject.toml
cp "$BACKUP_FILE" ~/.uvup/$ENV_NAME/pyproject.toml

# Sync dependencies
cd ~/.uvup/$ENV_NAME
uv lock
uv sync

echo "Restored $ENV_NAME from $BACKUP_FILE"
```

## Environment Maintenance

### Monthly Maintenance Script

```bash
#!/bin/bash
# monthly-maintenance.sh

# Update uvup itself
echo "Updating uvup..."
uvup update

# List all environments
echo "Current environments:"
uvup list

# Update templates with latest packages
for template in web-template ds-template cli-template; do
  if uvup list | grep -q "$template"; then
    echo "Updating $template..."
    uvup activate $template
    uvup lock --upgrade
    uv sync
    # Run tests if they exist
    if [ -d "tests" ]; then
      uv run pytest || echo "Tests failed for $template"
    fi
    uvup deactivate
  fi
done

echo "Maintenance complete!"
```

### Cleanup Old Environments

```bash
#!/bin/bash
# cleanup-old-environments.sh

# List all environments with modification time
for env in ~/.uvup/*/; do
  name=$(basename "$env")
  
  # Skip if it's a template
  if [[ $name == *"-template" ]]; then
    continue
  fi
  
  # Check if older than 30 days
  if [ -n "$(find "$env" -maxdepth 0 -mtime +30)" ]; then
    echo "Old environment: $name (>30 days)"
    read -p "Remove? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      uvup delete "$name"
      echo "Removed $name"
    fi
  fi
done
```

## Performance Optimization

### Parallel Environment Creation

```bash
#!/bin/bash
# create-environments-parallel.sh

# Array of environments to create
envs=("project-a" "project-b" "project-c" "project-d")
template="base-template"

# Create environments in parallel
for env in "${envs[@]}"; do
  (
    uvup new "$env" --template "$template"
    echo "Created $env"
  ) &
done

# Wait for all background jobs
wait
echo "All environments created!"
```

### Shared Cache

Leverage uv's cache for faster installs:

```bash
# uv automatically caches packages
# Multiple environments share the same cache

# First environment (downloads packages)
uvup create env1
uvup activate env1
uvup add numpy pandas  # Downloads and caches

# Second environment (uses cache)
uvup create env2
uvup activate env2
uvup add numpy pandas  # Much faster!
```
