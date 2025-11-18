# 虚拟环境

用于创建、列出和删除虚拟环境的命令。

## create

创建一个新的空虚拟环境。

### 用法

```bash
uvup create <name> [OPTIONS]
```

### 参数

- `<name>` - 要创建的环境名称

### 选项

- `-p, --python <version>` - Python 版本(默认: 3.12)

### 示例

```bash
# 使用默认 Python 创建环境
uvup create myproject

# 使用特定 Python 版本创建
uvup create myproject --python 3.11
uvup create --python 3.11 myproject
```

### 注意事项

- 创建具有最小配置的空 pyproject.toml
- 使用 uv 初始化虚拟环境
- 环境在 `~/.uvup/<name>/` 中创建

---

## list

列出所有可用的环境。

### 用法

```bash
uvup list
```

### 输出

- 列出 `~/.uvup/` 中的所有环境
- 如果为空则显示 "No environments found."

### 示例

```bash
uvup list
```

---

## delete

删除现有环境。

### 用法

```bash
uvup delete <name>
```

### 参数

- `<name>` - 要删除的环境名称

### 示例

```bash
uvup delete myproject
```

### 注意事项

- 永久删除环境目录
- 无法撤销
- 如果环境不存在则失败
- 一次只能删除一个环境

---

## clone

克隆现有环境以创建精确的 1:1 副本。

### 用法

```bash
uvup clone <source> <target>
```

### 参数

- `<source>` - 源环境名称
- `<target>` - 目标环境名称

### 示例

```bash
# 创建精确备份
uvup clone myproject myproject-backup

# 克隆用于测试
uvup clone production testing
```

### 克隆的内容

- `pyproject.toml` - 项目配置
- `hello.py` - 演示文件(如果存在)
- `uv.lock` - 锁文件(如果存在)
- 虚拟环境 - 带有相同包的全新 venv

### 不克隆的内容

- `.venv/` 目录(重新创建)
- 自定义文件(仅标准文件)

### 注意事项

- 纯 1:1 副本 **没有修改选项**
- 如果需要在复制期间修改,请使用 `new`
- 自动从锁文件同步包
