# 快速入门

本指南将引导你使用 uvup 创建第一个环境。

## 验证安装

安装 uvup 后(参见[安装指南](./installation.md)),验证其是否正常工作:

```bash
uvup --version
```

安装脚本会自动运行 `uvup init` 为所有检测到的 shell 进行配置。

如果你跳过了自动安装程序或需要重新配置,可以运行:

```bash
uvup init
```

详细信息请参阅下面的 [Shell 集成](#shell-integration) 部分。

## 基本工作流程

### 1. 创建环境

```bash
# 创建新环境
uvup create myproject

# 或者使用特定的 Python 版本
uvup create myproject --python 3.12
```

这将在 `~/.uvup/myproject` 创建一个新的虚拟环境,包含:
- 包含 Python 虚拟环境的 `.venv` 目录
- 用于依赖管理的 `pyproject.toml` 文件
- 用于可重现安装的 `uv.lock` 文件

### 3. 列出环境

```bash
uvup list
```

这将显示你创建的所有环境。

### 4. 激活环境

```bash
uvup activate myproject
```

激活后:
- 你的 shell 提示符显示 `(myproject)`
- Python 指向环境的 Python
- 你可以使用 uvup 包管理命令

### 5. 添加包

```bash
# 添加包到你的环境
uvup add numpy pandas requests

# 添加开发依赖
uvup add --group dev pytest black mypy
```

包会被添加到 `pyproject.toml` 并自动安装。

### 6. 使用你的代码

```bash
# 运行 Python 脚本
python script.py        # 经典用法
uv run script.py        # uv 风格用法

# 使用已安装的工具
pytest tests/           # 经典用法
uv run pytest tests/    # uv 风格用法

jupyter notebook        # 经典用法
uv run jupyter notebook # uv 风格用法
```

### 7. 管理依赖

```bash
# 更新锁文件
uvup lock

# 升级所有包
uvup lock --upgrade

# 查看依赖树
uvup tree

# 限制深度查看
uvup tree --depth 2
```

### 8. 删除包

```bash
# 删除一个包
uvup remove pandas

# 从特定组中删除
uvup remove --group dev pytest
```

### 9. 停用

```bash
uvup deactivate
```

这会将你的 shell 恢复到原始状态。

### 10. 删除环境

```bash
uvup delete myproject
```

这会永久删除环境目录。

## 示例会话

这是从头开始的完整示例工作流程:

```bash
# 创建数据科学环境
$ uvup create data-analysis --python 3.11
Environment 'data-analysis' created successfully

# 列出环境
$ uvup list
data-analysis

# 激活它
$ uvup activate data-analysis
(data-analysis) $

# 添加包
(data-analysis) $ uvup add numpy pandas matplotlib jupyter
Added: numpy, pandas, matplotlib, jupyter

# 运行你的分析
(data-analysis) $ python analyze.py        # 经典用法
(data-analysis) $ uv run analyze.py        # uv 风格用法

# 添加开发工具
(data-analysis) $ uvup add --group dev pytest black
Added to dev: pytest, black

# 完成后停用
(data-analysis) $ uvup deactivate
$

# 清理
$ uvup delete data-analysis
Environment 'data-analysis' removed successfully
```

## Shell 集成

`uvup init` 命令配置你的 shell 以启用 `uvup activate` 和 `uvup deactivate` 命令。

### 支持的 Shell

| 平台 | 自动检测的 Shell |
|----------|---------------------|
| **Windows** | PowerShell、Git Bash |
| **macOS** | Bash、Zsh、Fish |
| **Linux** | Bash、Zsh、Fish |

### 修改的配置文件

| Shell | 文件 |
|-------|------|
| Bash | `~/.bashrc` (在 Windows 上还会创建 `~/.bash_profile`) |
| Zsh | `~/.bashrc` |
| Fish | `~/.config/fish/config.fish` |
| PowerShell | `$PROFILE` |

### 手动配置

如果你需要重新配置或自定义:

```bash
# 初始化所有检测到的 shell
uvup init

# 仅初始化特定的 shell
uvup init powershell
uvup init bash

# 预览更改而不修改文件
uvup init --dry-run

# 获取 shell 脚本以进行手动设置
uvup init --raw

# 删除 shell 集成
uvup init --reverse
```

运行 `uvup init` 后,重新启动终端或重新加载你的 shell:

```bash
source ~/.bashrc  # Bash
source ~/.zshrc   # Zsh
source ~/.config/fish/config.fish  # Fish
# PowerShell: 只需重新启动终端
```

## 故障排除

### 找不到 `uvup activate`

这意味着 shell 集成没有正确设置。运行:

```bash
uvup init
```

然后重新启动你的终端。

### 更改未生效

确保在运行 `uvup init` 后重新启动了终端或重新加载了 shell 配置。

## 下一步

- [核心概念](./core-concepts.md) - 理解 uvup 的设计理念
- [命令参考](./commands/README.md) - 完整的命令文档
- [使用案例](./use-cases/README.md) - 实际使用场景
