# 安装

## (前提条件)

uvup 需要安装 [uv](https://github.com/astral-sh/uv):

```bash
# 如果你还没有安装 uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# 在 Windows 上:
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

## 快速安装

### Linux 和 macOS

```bash
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.sh | sh
```

### Windows (PowerShell)

```powershell
Invoke-RestMethod https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/install.ps1 | Invoke-Expression
```

## 安装程序的操作

安装脚本将:

1. 为你的平台下载最新的 uvup 二进制文件
2. 安装到:
   - Linux/macOS: `~/.local/bin/uvup` (或 `/usr/local/bin` 如果可写)
   - Windows: `%LOCALAPPDATA%\Programs\uvup\uvup.exe`
3. 将安装目录添加到你的 PATH
4. 自动运行 `uvup init` 配置 shell 集成

## 验证安装

检查 uvup 是否正确安装:

```bash
uvup --version
```

## 配置

### 自定义环境存储位置

默认情况下，`uvup` 将环境存储在 `~/.uvup` (Linux/macOS) 或 `%USERPROFILE%\.uvup` (Windows) 中。你可以通过设置 `UVUP_HOME` 环境变量来自定义此位置。

**Bash/Zsh:**
```bash
export UVUP_HOME="/path/to/your/envs"
```

**PowerShell:**
```powershell
[System.Environment]::SetEnvironmentVariable("UVUP_HOME", "D:\MyPyEnvs", "User")
```

**注意：** 设置此变量后，你可能需要重启终端或重新运行 `uvup init` 以便让 Shell 集成脚本生效。

## 从源代码构建

用于开发或自定义构建:

```bash
# 克隆仓库
git clone https://github.com/KercyDing/uvup.git
cd uvup

# 构建发布版本
cargo build --release

# 二进制文件将位于 target/release/uvup
# 手动复制到你的 PATH 并运行 'uvup init'
```

## 卸载

从系统中删除 uvup:

### Linux 和 macOS

```bash
curl -fsSL https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/uninstall.sh | sh -s -- -y
```

### Windows (PowerShell)

```powershell
Invoke-RestMethod https://raw.githubusercontent.com/KercyDing/uvup/main/scripts/uninstall.ps1 | Invoke-Expression
```

卸载脚本将:
- 删除 shell 集成 (`uvup init --reverse`)
- 删除 uvup 二进制文件
- 从 PATH 中删除
- 删除所有环境

### 手动卸载

如果你更喜欢手动卸载:

```bash
# 1. 删除 shell 集成
uvup init --reverse

# 2. 删除所有环境(可选)
rm -rf ~/.uvup  # Linux/macOS
Remove-Item -Recurse -Force "$env:USERPROFILE\.uvup"  # Windows
```

## 更新

将 uvup 更新到最新版本:

```bash
# 更新到最新版本
uvup update

# 检查是否有可用更新
uvup update --check
```

更新命令将:
- 在 GitHub 发布中检查最新版本
- 为你的平台下载合适的二进制文件
- 原地替换当前二进制文件
- 保留所有环境和配置

**注意:** 更新后重新启动终端以使用新版本。

## 下一步

继续阅读[快速入门](./quick-start.md)开始使用 uvup!
