# Command Reference

Complete reference for all uvup commands.

## Command Categories

### Environment Management

Basic CRUD operations for virtual environments:

- [create](./environment.md#create) - Create new empty environment
- [list](./environment.md#list) - List all environments
- [delete](./environment.md#delete) - Delete an environment
- [clone](./environment.md#clone) - Clone environment (1:1 copy)

### Project Management

Template-based project workflows:

- [new](./project.md#new) - Create project from template
- [sync](./project.md#sync) - Sync project with template

### Package Management

Manage packages in activated environments:

- [add](./package.md#add) - Add packages
- [remove](./package.md#remove) - Remove packages
- [lock](./package.md#lock) - Update lockfile
- [tree](./package.md#tree) - Display dependency tree

### System

System-level operations:

- [init](./system.md#init) - Initialize shell integration
- [update](./system.md#update) - Update uvup itself

### Shell Commands

Shell integration commands:

- [activate](./shell.md#activate) - Activate environment
- [deactivate](./shell.md#deactivate) - Deactivate environment

## Command Decision Tree

**Need to create something?**
- Empty environment → `create`
- Exact copy → `clone`
- New project from template → `new`

**Need to update?**
- Current project from template → `sync`
- uvup itself → `update`

**Need to manage environments?**
- See all environments → `list`
- Delete environment → `delete`

**Need to manage packages?** (requires activation)
- Add packages → `add`
- Remove packages → `remove`
- Update lockfile → `lock`
- View dependencies → `tree`

**Need to use?**
- Enable activation → `init`
- Enter environment → `activate`
- Exit environment → `deactivate`
