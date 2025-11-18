# 工作流程

日常开发的常见工作流程和使用模式。

## 快速项目创建

### 数据科学项目

```bash
uvup create data-project
uvup activate data-project
uvup add numpy pandas matplotlib jupyter scikit-learn
```

### Web API 项目

```bash
uvup create api-project
uvup activate api-project
uvup add fastapi uvicorn pydantic sqlalchemy
uvup add --group dev pytest black mypy
```

### CLI 工具项目

```bash
uvup create cli-tool
uvup activate cli-tool
uvup add click rich typer
uvup add --group dev pytest
```

## 使用模板

### 创建模板

创建可复用的环境配置:

```bash
# 创建 web 模板
uvup create web-template
uvup activate web-template
uvup add fastapi uvicorn sqlalchemy pydantic
uvup add --group dev pytest black mypy ruff
```

### 从模板启动新项目

```bash
# 从模板创建新项目
uvup new myapi --template web-template

# 使用自定义 Python 版本
uvup new myapi --template web-template --python 3.11

# 排除开发工具
uvup new myapi --template web-template --exclude pytest,black,mypy
```

### 创建前预览

```bash
# 查看将创建什么
uvup new myapi --template web-template --dry-run
```

## 同步现有项目

从模板更新现有项目:

```bash
# 基本同步
cd myproject
uvup sync --template web-template

# 首先预览更改
uvup sync --template web-template --dry-run

# 选择性同步(排除某些包)
uvup sync --template web-template --exclude pytest,black
```

## 管理多个项目

```bash
# 列出所有环境
uvup list

# 在项目之间切换
uvup activate project-a
# ... 在 project-a 上工作 ...
uvup deactivate

uvup activate project-b
# ... 在 project-b 上工作 ...
uvup deactivate
```

## 安全地实验

在进行更改之前克隆环境:

```bash
# 克隆生产环境
uvup clone production-env experiment-env

# 激活并测试
uvup activate experiment-env
uvup add experimental-package
uv run python -m pytest       # 经典用法
uv run python -m pytest # uv 风格用法
uvup deactivate

# 如果成功,更新生产环境
uvup activate production-env
uvup add experimental-package
uvup deactivate

# 如果失败,删除实验
uvup delete experiment-env
```

## 升级依赖

安全升级策略:

```bash
# 1. 创建备份
uvup clone my-project my-project-backup

# 2. 在原始环境中升级
uvup activate my-project
uvup lock --upgrade
uv sync
uv run python -m pytest
uvup deactivate

# 3. 如果成功,清理备份
uvup delete my-project-backup

# 4. 如果失败,从备份恢复
uvup delete my-project
uvup clone my-project-backup my-project
uvup delete my-project-backup
```

## 团队协作

### 共享环境定义

创建团队模板:

```bash
# 在一个团队成员的机器上
uvup create team-template
uvup activate team-template
uvup add requests fastapi sqlalchemy pydantic
uvup add --group dev pytest black mypy
uvup deactivate

# 共享 pyproject.toml 和 uv.lock
git add pyproject.toml uv.lock
git commit -m "Add team environment template"
git push
```

### 团队成员设置

```bash
# 其他团队成员
git pull
uvup create my-project
cd my-project
cp ../team-template/pyproject.toml .
cp ../team-template/uv.lock .
uv sync
```

### 代码审查环境

```bash
# 审查者创建临时环境
uvup clone main-env pr-123-review
uvup activate pr-123-review

# 应用 PR 更改
git checkout pr-branch

# 如果依赖更改则同步
uvup sync --template main-template

# 测试更改
uv run python -m pytest      # 经典用法
uv run python -m pytest # uv 风格用法
uvup deactivate

# 清理
uvup delete pr-123-review
```

## 最佳实践

### 模板命名

```bash
# 使用描述性名称
uvup create web-template        # 好
uvup create ml-template         # 好
uvup create template1           # 不好
```

### 定期更新

```bash
# 定期更新模板
uvup activate web-template
uvup lock --upgrade
uv sync

# 从更新的模板同步项目
cd myproject
uvup sync --template web-template
```

### 锁文件策略

始终提交 `uv.lock` 以实现可重现的环境:

```bash
git add pyproject.toml uv.lock
git commit -m "Update dependencies"
```
