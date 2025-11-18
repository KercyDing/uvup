# 包

在激活的环境中管理包。所有命令都需要激活的环境并可从任何目录工作。

## add

向活动环境添加包。

### 用法

```bash
uvup add <packages...> [OPTIONS]
```

### 参数

- `<packages...>` - 要添加的一个或多个包

### 选项

- `--group <name>` - 添加到可选依赖组

### 示例

```bash
# 首先激活环境
uvup activate myproject

# 添加包
uvup add requests numpy pandas

# 使用版本说明符添加
uvup add "requests>=2.28.0" "numpy<2.0"

# 添加到开发组
uvup add --group dev pytest black mypy
```

### 注意事项

- 需要激活的环境
- 更新 `pyproject.toml` 和 `uv.lock`
- 立即安装包
- 从任何目录工作(不仅仅是项目根目录)

---

## remove

从活动环境中删除包。

### 用法

```bash
uvup remove <packages...> [OPTIONS]
```

### 参数

- `<packages...>` - 要删除的一个或多个包

### 选项

- `--group <name>` - 从可选依赖组中删除

### 示例

```bash
# 首先激活环境
uvup activate myproject

# 删除包
uvup remove requests numpy

# 从开发组中删除
uvup remove --group dev pytest
```

### 注意事项

- 需要激活的环境
- 更新 `pyproject.toml` 和 `uv.lock`
- 立即卸载包
- 从任何目录工作(不仅仅是项目根目录)

---

## lock

更新活动环境的锁文件。

### 用法

```bash
uvup lock [OPTIONS]
```

### 选项

- `--upgrade` - 将所有包升级到最新的兼容版本

### 示例

```bash
# 首先激活环境
uvup activate myproject

# 更新锁文件
uvup lock

# 升级所有包
uvup lock --upgrade
```

### 注意事项

- 需要激活的环境
- 基于 `pyproject.toml` 更新 `uv.lock`
- 不安装包(使用 `uv sync` 安装)
- 从任何目录工作(不仅仅是项目根目录)

---

## tree

显示活动环境的依赖树。

### 用法

```bash
uvup tree [OPTIONS]
```

### 选项

- `--depth <n>` - 要显示的最大深度

### 示例

```bash
# 首先激活环境
uvup activate myproject

# 显示完整的依赖树
uvup tree

# 限制深度
uvup tree --depth 2
```

### 注意事项

- 需要激活的环境
- 显示依赖的层次结构视图
- 帮助识别依赖冲突
- 从任何目录工作(不仅仅是项目根目录)
