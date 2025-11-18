# 核心概念

理解 uvup 的设计理念和关键功能。

## 设计理念

uvup 通过以下方式增强 uv:
- **集中的环境存储** 在 `~/.uvup/`
- **类似 Conda 的激活工作流** 便于环境切换
- **基于模板的项目创建** 用于可复用配置

如果你来自 conda,uvup 会让你感觉很自然:

```bash
# conda 工作流
conda create -n myenv python=3.11
conda activate myenv
conda install numpy

# uvup 工作流
uvup create myenv --python 3.11
uvup activate myenv
uvup add numpy
```

## 关键功能

### 集中管理

所有环境都在 `~/.uvup/` 中,而不是分散在项目目录中。

### 位置独立

激活后可以从任何目录管理包:

```bash
uvup activate myproject
cd ~/anywhere
uvup add requests  # 有效!
```

### 模板系统

复用环境配置:

```bash
# 创建模板
uvup create web-template
uvup activate web-template
uvup add fastapi uvicorn

# 用于新项目
uvup new myapp --template web-template
```

### 四种工作流程

1. **环境管理**: `create`、`list`、`delete`
2. **克隆**: `clone source target` - 精确的 1:1 复制
3. **项目创建**: `new` - 从模板创建并可自定义
4. **同步**: `sync` - 从模板更新现有项目

## 依赖管理

使用现代 Python 标准:
- `pyproject.toml` 用于依赖
- `uv.lock` 用于可重现的安装
- 可选的依赖组

```bash
uvup add requests              # 主要
uvup add --group dev pytest    # 开发
uvup add --group docs sphinx   # 文档
```

## 与其他工具的比较

| 工具 | uvup 的优势 | 何时使用其他工具 |
|------|----------------|----------------------|
| **conda** | 更快(使用 uv)、仅 Python | 需要非 Python 包 |
| **virtualenv/venv** | 集中化、模板、类似 conda | 简单的每个项目需求 |
| **直接使用 uv** | Shell 集成、模板 | CI/CD、每个项目工作流 |

## 最佳实践

**使用描述性名称:**
```bash
uvup create data-analysis  # 好
uvup create env1           # 不好
```

**使用模板组织:**
```bash
uvup create web-template
uvup create ds-template
uvup create ml-template
```

**使用依赖组:**
```bash
uvup add requests pydantic              # 核心
uvup add --group dev pytest black mypy  # 开发
uvup add --group docs sphinx            # 文档
```

## 下一步

- [命令参考](../commands/index.md) - 完整的命令文档
- [使用案例](../use-cases/index.md) - 实际示例
