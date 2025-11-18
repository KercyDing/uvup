# 命令

所有 uvup 命令的完整参考。

## 命令分类

### 虚拟环境

虚拟环境的基本 CRUD 操作:

- [create](./environment.md#create) - 创建新的空环境
- [list](./environment.md#list) - 列出所有环境
- [delete](./environment.md#delete) - 删除环境
- [clone](./environment.md#clone) - 克隆环境(1:1 复制)

### 项目

基于模板的项目工作流:

- [new](./project.md#new) - 从模板创建项目
- [sync](./project.md#sync) - 将项目与模板同步

### 包

在激活的环境中管理包:

- [add](./package.md#add) - 添加包
- [remove](./package.md#remove) - 删除包
- [lock](./package.md#lock) - 更新锁文件
- [tree](./package.md#tree) - 显示依赖树

### Shell

Shell 集成和激活:

- [init](./shell.md#init) - 初始化 shell 集成
- [activate](./shell.md#activate) - 激活环境
- [deactivate](./shell.md#deactivate) - 停用环境

## 命令决策树

**需要创建什么?**
- 空环境 → `create`
- 精确复制 → `clone`
- 从模板创建新项目 → `new`

**需要更新?**
- 从模板更新当前项目 → `sync`
- 更新 uvup 本身 → `update` (参见[安装](../installation.md#update))

**需要管理环境?**
- 查看所有环境 → `list`
- 删除环境 → `delete`

**需要管理包?** (需要激活)
- 添加包 → `add`
- 删除包 → `remove`
- 更新锁文件 → `lock`
- 查看依赖 → `tree`

**需要使用?**
- 启用激活 → `init`
- 进入环境 → `activate`
- 退出环境 → `deactivate`
