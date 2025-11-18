# 项目

基于模板的项目创建和同步。

## new

从模板环境创建新项目,支持修改。

### 用法

```bash
uvup new <name> --template <template> [OPTIONS]
```

### 参数

- `<name>` - 项目名称
- `--template <template>` - 模板环境名称

### 选项

- `-p, --python <version>` - 覆盖 Python 版本
- `--exclude <packages>` - 排除包(逗号分隔)
- `--include <packages>` - 仅包含这些包(逗号分隔)
- `--path <directory>` - 在自定义目录中创建(默认: 当前目录)
- `--dry-run` - 预览更改而不创建

### 示例

```bash
# 基本项目创建
uvup new myapp --template web-template

# 自定义 Python 版本
uvup new myapp --template web-template --python 3.11

# 排除开发工具
uvup new myapp --template web-template --exclude pytest,black,mypy

# 仅包含特定包
uvup new myapp --template web-template --include numpy,pandas,requests

# 在特定目录中创建
uvup new myapp --template web-template --path ~/projects

# 预览而不创建
uvup new myapp --template web-template --exclude pytest --dry-run
```

### 修改行为

1. **项目名称**: 在 pyproject.toml 中自动更新
2. **Python 版本**: 如果指定了 `--python` 则覆盖
3. **依赖**: 通过 `--exclude` 或 `--include` 过滤
4. **可选依赖**: 同样会过滤,空组会被删除

### 过滤规则

- `--exclude`: 从主依赖和可选依赖中删除指定的包
- `--include`: 仅保留指定的包(删除所有其他包)
- 不能同时使用 `--exclude` 和 `--include`
- 包名不区分大小写
- 处理 PEP 508 说明符(例如 `requests[http3]>=2.0`)

### 试运行输出

```
-- Dry Run Mode --

Template: 'web-template' (Python 3.12)
Project:  'myapp' (Python 3.11)
Location: /Users/you/myapp

Python version change:
  3.12 → 3.11

Filters applied:
  Exclude: pytest, black

Dependency changes:
  Removed (2):
    - pytest>=7.0.0
    - black>=23.0.0
  Kept (5):

Optional dependencies:
  [dev]: Removed (group is empty after filtering)
  [viz]: No changes

To create this project, run the same command without --dry-run
```

### 注意事项

- 在 `<path>/<name>/` 中创建项目(默认: `./<name>/`)
- 如果目标目录已存在则失败
- 自动运行 `uv lock` 和 `uv sync`
- 项目立即可用

---

## sync

将当前项目与模板环境同步。

### 用法

```bash
uvup sync --template <template> [OPTIONS]
```

### 参数

- `--template <template>` - 模板环境名称

### 选项

- `-p, --python <version>` - 覆盖 Python 版本
- `--exclude <packages>` - 排除包(逗号分隔)
- `--include <packages>` - 仅包含这些包(逗号分隔)
- `--dry-run` - 预览更改而不同步

### 示例

```bash
# 与模板同步
cd myproject
uvup sync --template web-template

# 同步并更改 Python 版本
uvup sync --template web-template --python 3.11

# 同步但不包含开发依赖
uvup sync --template web-template --exclude pytest,black,mypy

# 仅同步核心包
uvup sync --template web-template --include numpy,pandas,requests

# 预览更改
uvup sync --template web-template --dry-run
```

### 同步行为

1. **依赖**: 替换为模板的依赖(已过滤)
2. **可选依赖**: 替换为模板的可选依赖(已过滤)
3. **Python 版本**: 如果指定了 `--python` 则更新
4. **项目名称**: 保留(不更改)

### 安全功能

- 自动备份: 在更改前创建 `pyproject.toml.backup`
- 错误回滚: 如果 `uv lock` 或 `uv sync` 失败则恢复备份
- 备份清理: 成功完成时删除

### 试运行输出

```
-- Dry Run Mode --

Template: 'web-template' (Python 3.12)
Current:  /Users/you/myproject (Python 3.12)

Dependency changes:
  Added (2):
    + fastapi>=0.100.0
    + uvicorn>=0.23.0
  Removed (1):
    - flask>=2.3.0
  Kept (3):

Optional dependencies:
  [dev]: Modified (5 packages)
  [viz]: No changes

To sync this project, run the same command without --dry-run
```

### 注意事项

- 必须从包含 `pyproject.toml` 的项目目录运行
- 如果找不到 `pyproject.toml` 则失败
- 更改立即生效(试运行模式除外)
- 如果用户中断(Ctrl+C),备份文件会保留
