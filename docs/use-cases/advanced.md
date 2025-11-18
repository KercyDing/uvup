# Advanced

Real-world integration scenarios combining uvup with other tools and workflows.

## CI/CD Integration

### GitHub Actions with uvup

Use uvup for consistent testing across branches:

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install uv
        run: curl -LsSf https://astral.sh/uv/install.sh | sh

      - name: Install uvup
        run: curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh

      - name: Setup test environment
        run: |
          uvup create ci-test
          uvup activate ci-test

          # Install from lockfile for reproducibility
          cp pyproject.toml uv.lock ~/.uvup/ci-test/
          cd ~/.uvup/ci-test && uv sync --frozen

      - name: Run tests
        run: |
          uvup activate ci-test
          uv run pytest tests/

      - name: Run linters
        run: |
          uvup activate ci-test
          uv run ruff check .
          uv run black --check .
```

**Why uvup here?**
- Centralized environment in `~/.uvup/` regardless of working directory
- Can run commands from any path after activation
- Easy to recreate exact same environment locally for debugging

---

## Docker Multi-Stage Builds

### Optimized Production Images

Leverage uvup templates for clean production builds:

```dockerfile
# Dockerfile
FROM python:3.12-slim as builder

# Install uv and uvup
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
RUN curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
ENV PATH="/root/.local/bin:$PATH"

# Create production environment (no dev dependencies)
RUN uvup create prod-template
WORKDIR /root/.uvup/prod-template

# Copy dependency files
COPY pyproject.toml uv.lock ./

# Install only production dependencies
RUN uv sync --frozen --no-dev

# ---- Production Stage ----
FROM python:3.12-slim

# Copy only virtual environment
COPY --from=builder /root/.uvup/prod-template/.venv /app/.venv

# Copy application code
COPY . /app
WORKDIR /app

# Use virtual environment
ENV PATH="/app/.venv/bin:$PATH"

CMD ["python", "main.py"]
```

**Build with different profiles:**

```bash
# Development image (with debug tools)
docker build --target builder -t myapp:dev .

# Production image (minimal)
docker build -t myapp:prod .
```

---

## VSCode Dev Containers

### Team Development Standardization

Use uvup in dev containers for consistent team environments:

```json
// .devcontainer/devcontainer.json
{
  "name": "Python Project",
  "image": "mcr.microsoft.com/devcontainers/python:3.12",

  "features": {
    "ghcr.io/devcontainers/features/common-utils:2": {}
  },

  "postCreateCommand": "bash .devcontainer/setup.sh",

  "customizations": {
    "vscode": {
      "extensions": [
        "ms-python.python",
        "charliermarsh.ruff"
      ],
      "settings": {
        "python.defaultInterpreterPath": "${env:HOME}/.uvup/devcontainer/.venv/bin/python"
      }
    }
  }
}
```

```bash
# .devcontainer/setup.sh
#!/bin/bash

# Install uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# Install uvup
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh

# Initialize shell
uvup init

# Create team environment from template
uvup create devcontainer
cd ~/.uvup/devcontainer

# Copy project config
cp /workspaces/myproject/pyproject.toml .
cp /workspaces/myproject/uv.lock .

# Install dependencies
uv sync

echo "✓ Development environment ready!"
echo "Run: uvup activate devcontainer"
```

**Benefits:**
- Every team member gets identical environment
- New developers onboard in minutes
- Easy to update team-wide: just update template

---

## Pre-commit Hooks Integration

### Automated Environment Validation

Ensure environment consistency before commits:

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: check-lockfile
        name: Check uv.lock is up to date
        entry: bash -c 'uvup activate dev && uv lock --check'
        language: system
        pass_filenames: false

      - id: lint
        name: Run ruff
        entry: bash -c 'uvup activate dev && uv run ruff check .'
        language: system
        types: [python]
        pass_filenames: false
```

Setup:

```bash
# Create dev environment
uvup create dev
uvup activate dev
uvup add --group dev pre-commit ruff black

# Install pre-commit hooks
uv run pre-commit install
```

Now before every commit:
1. Checks if `uv.lock` is synchronized with `pyproject.toml`
2. Runs linters using the dev environment
3. Prevents commits if checks fail
