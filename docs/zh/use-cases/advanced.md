# 高级用法

结合其他工具和工作流的实际应用场景。

## CI/CD 集成

### GitHub Actions 中使用 uvup

使用 uvup 实现跨分支的一致性测试：

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

          # 从锁文件安装以确保可复现性
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

**为什么在这里用 uvup？**
- 环境集中在 `~/.uvup/`，不受工作目录影响
- 激活后可以从任意路径运行命令
- 容易在本地重现完全相同的环境来调试

---

## Docker 多阶段构建

### 优化的生产镜像

利用 uvup 模板构建干净的生产镜像：

```dockerfile
# Dockerfile
FROM python:3.12-slim as builder

# 安装 uv 和 uvup
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
RUN curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
ENV PATH="/root/.local/bin:$PATH"

# 创建生产环境（不含开发依赖）
RUN uvup create prod-template
WORKDIR /root/.uvup/prod-template

# 复制依赖文件
COPY pyproject.toml uv.lock ./

# 只安装生产依赖
RUN uv sync --frozen --no-dev

# ---- 生产阶段 ----
FROM python:3.12-slim

# 只复制虚拟环境
COPY --from=builder /root/.uvup/prod-template/.venv /app/.venv

# 复制应用代码
COPY . /app
WORKDIR /app

# 使用虚拟环境
ENV PATH="/app/.venv/bin:$PATH"

CMD ["python", "main.py"]
```

**构建不同配置：**

```bash
# 开发镜像（带调试工具）
docker build --target builder -t myapp:dev .

# 生产镜像（最小化）
docker build -t myapp:prod .
```

---

## VSCode Dev Containers

### 团队开发标准化

在 dev containers 中使用 uvup 确保团队环境一致：

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

# 安装 uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# 安装 uvup
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh

# 初始化 shell
uvup init

# 从模板创建团队环境
uvup create devcontainer
cd ~/.uvup/devcontainer

# 复制项目配置
cp /workspaces/myproject/pyproject.toml .
cp /workspaces/myproject/uv.lock .

# 安装依赖
uv sync

echo "✓ 开发环境就绪！"
echo "运行: uvup activate devcontainer"
```

**好处：**
- 每个团队成员获得相同环境
- 新开发者几分钟内就能上手
- 易于团队范围更新：只需更新模板

---

## Pre-commit Hooks 集成

### 自动化环境验证

确保提交前环境一致性：

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

设置：

```bash
# 创建开发环境
uvup create dev
uvup activate dev
uvup add --group dev pre-commit ruff black

# 安装 pre-commit hooks
uv run pre-commit install
```

现在每次提交前会自动：
1. 检查 `uv.lock` 是否与 `pyproject.toml` 同步
2. 使用 dev 环境运行代码检查
3. 如果检查失败则阻止提交
