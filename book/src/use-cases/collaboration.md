# Team Collaboration

Working with teams using uvup.

## Sharing Environment Definitions

**Scenario**: Team wants consistent development environments.

### Creating a Team Template

```bash
# Team lead creates template
uvup create team-base-template --python 3.11
uvup activate team-base-template
uvup add requests httpx pydantic sqlalchemy
uvup add --group dev pytest black mypy ruff pre-commit
uvup add --group docs sphinx sphinx-rtd-theme
uvup deactivate

# Team lead shares the pyproject.toml
cp ~/.uvup/team-base-template/pyproject.toml ./templates/team-base.toml
git add templates/team-base.toml
git commit -m "Add team base template"
git push
```

### Team Members Setup

```bash
# Clone the repository
git clone https://github.com/company/project
cd project

# Create template from shared config
uvup create team-base-template
cp ./templates/team-base.toml ~/.uvup/team-base-template/pyproject.toml
cd ~/.uvup/team-base-template
uv lock
uv sync
cd -

# Now create projects from template
uvup new my-feature --template team-base-template
```

## Onboarding New Developers

### Automated Onboarding Script

```bash
#!/bin/bash
# onboard.sh - Run this to set up new developer

echo "Installing uvup..."
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh

echo "Initializing shell..."
uvup init
source ~/.bashrc  # or ~/.zshrc

echo "Creating templates..."
for template in backend frontend ml; do
  uvup create ${template}-template
  cp templates/${template}.toml ~/.uvup/${template}-template/pyproject.toml
  cd ~/.uvup/${template}-template && uv sync && cd -
done

echo "Done! Available templates:"
uvup list
```

### Documentation for New Team Members

Create a SETUP.md in your repository:

```markdown
# Development Setup

## Prerequisites

1. Install uvup:
   ```bash
   curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
   ```

2. Initialize shell integration:
   ```bash
   uvup init
   source ~/.bashrc  # or your shell config
   ```

## Quick Start

1. Run the onboarding script:
   ```bash
   ./scripts/onboard.sh
   ```

2. Create your working environment:
   ```bash
   uvup new my-work --template backend-template
   cd my-work
   ```

3. Start developing!
```

## Consistent Environments Across Team

### Lock File Strategy

```bash
# Team lead creates and locks dependencies
uvup create shared-env
uvup activate shared-env
uvup add requests pandas numpy
uvup lock  # Creates uv.lock

# Commit both files
git add ~/.uvup/shared-env/pyproject.toml
git add ~/.uvup/shared-env/uv.lock
git commit -m "Lock dependencies for team"
```

Team members use the locked versions:

```bash
# Pull latest
git pull

# Update local template
uvup create shared-env
cp project/templates/pyproject.toml ~/.uvup/shared-env/
cp project/templates/uv.lock ~/.uvup/shared-env/
cd ~/.uvup/shared-env
uv sync --locked  # Install exact versions
```

## Code Review Environments

### Reviewing Pull Requests

```bash
# Create clean environment for PR review
uvup clone main-env pr-123-review
uvup activate pr-123-review

# Checkout PR branch
git fetch origin pull/123/head:pr-123
git checkout pr-123

# Sync with any new dependencies
uvup sync --template main-template

# Test the changes
python -m pytest
uvup deactivate

# Clean up after review
uvup delete pr-123-review
```

## Shared Template Repository

### Repository Structure

```
team-templates/
├── README.md
├── web-api.toml
├── data-pipeline.toml
├── ml-training.toml
└── scripts/
    └── sync-templates.sh
```

### Sync Script

```bash
#!/bin/bash
# sync-templates.sh - Update local templates from repository

TEMPLATE_REPO="https://github.com/company/team-templates"
TEMP_DIR=$(mktemp -d)

git clone "$TEMPLATE_REPO" "$TEMP_DIR"

for toml in "$TEMP_DIR"/*.toml; do
  name=$(basename "$toml" .toml)
  template="${name}-template"
  
  echo "Updating $template..."
  
  # Create if doesn't exist
  if ! uvup list | grep -q "$template"; then
    uvup create "$template"
  fi
  
  # Update pyproject.toml
  cp "$toml" ~/.uvup/$template/pyproject.toml
  cd ~/.uvup/$template
  uv lock
  uv sync
  cd -
done

rm -rf "$TEMP_DIR"
echo "Templates updated!"
```

## Cross-Platform Considerations

### Platform-Specific Dependencies

```bash
# Template for cross-platform project
uvup create cross-platform-template
uvup activate cross-platform-template

# Core dependencies (all platforms)
uvup add requests pydantic

# Platform-specific in pyproject.toml
# [project.dependencies]
# requests = ">=2.28.0"
# pydantic = ">=2.0.0"
# pywin32 = {version = ">=305", markers = "platform_system == 'Windows'"}
# pyobjc = {version = ">=9.0", markers = "platform_system == 'Darwin'"}
```

### Testing on Multiple Platforms

```bash
# CI/CD configuration example (.github/workflows/test.yml)
# Test on Linux, macOS, Windows

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v3
      - name: Install uvup
        run: |
          curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
      - name: Create environment
        run: |
          uvup create test-env
          cp pyproject.toml ~/.uvup/test-env/
          cd ~/.uvup/test-env && uv sync
      - name: Run tests
        run: cd ~/.uvup/test-env && uv run pytest
```
