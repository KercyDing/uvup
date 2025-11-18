# Shell

Shell 集成命令和初始化。

## init

初始化 uvup shell 集成。

### 用法

```bash
# 自动设置 - 初始化所有检测到的 shell
uvup init

# 初始化特定 shell
uvup init <shell>

# 选项
uvup init --raw          # 打印 shell 脚本而不是安装
uvup init --reverse      # 删除 uvup 初始化
uvup init --dry-run      # 预览更改而不修改文件
```

### 支持的 Shell

| 平台 | 自动检测的 Shell |
|----------|---------------------|
| **Windows** | PowerShell、Git Bash |
| **macOS** | Bash、Zsh、Fish |
| **Linux** | Bash、Zsh、Fish |

### 示例

```bash
# 自动配置所有检测到的 shell
uvup init

# 仅初始化 PowerShell
uvup init powershell

# 仅初始化 Bash
uvup init bash

# 预览将被更改的内容
uvup init --dry-run

# 删除 shell 集成
uvup init --reverse

# 获取原始 shell 脚本以进行手动设置
uvup init --raw
```

### 它做了什么

**自动模式** (`uvup init`):
1. 检测系统上所有已安装的 shell
2. 在每个 shell 的配置文件中添加初始化代码
3. 如果配置文件不存在则创建

**修改的配置文件:**

| Shell | 文件 |
|-------|------|
| Bash | `~/.bashrc` (在 Windows 上还会创建 `~/.bash_profile`) |
| Zsh | `~/.bashrc` |
| Fish | `~/.config/fish/config.fish` |
| PowerShell | `$PROFILE` |

**手动模式** (`uvup init --raw`):
- 打印当前 shell 的 shell 钩子脚本
- 用于自定义设置或 CI/CD 环境

### 运行后

你需要重新加载 shell 配置:

```bash
# Bash
source ~/.bashrc

# Zsh
source ~/.zshrc

# Fish
source ~/.config/fish/config.fish

# PowerShell
# 只需重新启动终端
```

或者简单地 **重新启动终端**。

### 注意事项

- 安装后运行一次
- 可以安全地多次运行(幂等)
- 在配置文件中使用标记的部分以便轻松删除
- 不修改现有配置

---

## activate

激活虚拟环境。

### 用法

```bash
uvup activate <name>
```

### 参数

- `<name>` - 要激活的环境名称

### 示例

```bash
uvup activate myproject
```

### 它做了什么

1. 检查环境是否存在
2. 激活环境中的 `.venv`
3. 修改 shell 提示符以显示环境名称
4. 启用包命令

### 效果

激活后:

- Shell 提示符显示 `(myproject)`
- `python` 指向环境的 Python
- `pip` 和其他工具使用环境的包
- 包命令(`add`、`remove`、`lock`、`tree`)已启用
- 从任何目录工作

### 注意事项

- 需要先设置 `uvup init`
- 一次只能激活一个环境
- 激活新环境会自动停用当前环境

---

## deactivate

停用当前虚拟环境。

### 用法

```bash
uvup deactivate
```

### 示例

```bash
uvup deactivate
```

### 它做了什么

1. 停用当前虚拟环境
2. 恢复原始 shell 提示符
3. 恢复原始 PATH
4. 禁用包管理命令

### 效果

停用后:

- Shell 提示符恢复正常
- `python` 指向系统 Python
- 包命令将失败并显示 "No active environment" 错误

### 注意事项

- 需要先设置 `uvup init`
- 即使没有激活的环境也可以安全运行
- 不删除或修改环境
