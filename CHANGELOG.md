# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-11-19

### Added

- **Package Management**: Integrated commands working with active environment
  - `uvup add/remove/lock/tree` - Manage packages from any directory after activation
- **Enhanced Init Command**: Multi-shell configuration management
  - `uvup init [shell]` - Initialize specific shell or auto-detect all shells
  - `--raw` - Print integration script without installing
  - `--reverse` - Remove uvup from shell configurations
  - `--dry-run` - Preview changes before applying
- **VitePress Documentation**: Professional bilingual documentation site
  - English and Chinese support with unified sidebar navigation
  - Real-world integration examples (CI/CD, Docker, VSCode, Pre-commit Hooks)
  - GitHub Pages deployment with pnpm

### Changed

- **Breaking**: Renamed `uvup remove <name>` to `uvup delete <name>` for environment deletion
  - `remove` is now exclusively for package removal
- Migrated from mdBook to VitePress for better UI and i18n support

### Fixed

- Shell activation script paths on Windows (Scripts/ vs bin/)
- UVUP_ACTIVE_ENV environment variable tracking across shells

### Technical

- Added `commands/{add,delete,lock,tree}.rs` for package management
- Enhanced `commands/init.rs` with multi-shell support (579 lines)
- Created VitePress configuration with bilingual sidebar structure
- Updated GitHub Actions workflow for VitePress deployment

## [0.2.0] - 2025-11-17

### Added

- **Four-Method Command Architecture**: Clear separation of responsibilities
  - `uvup clone <source> <target>` - Create exact 1:1 copy of an environment
  - `uvup new <name> --template <template>` - Create new project from template with modification support
  - `uvup sync --template <template>` - Sync current project with template
- **Template System**: Powerful project scaffolding capabilities
  - Python version override with `--python <version>`
  - Package filtering with `--exclude <packages>` and `--include <packages>`
  - Custom project path with `--path <directory>`
  - Dry-run preview mode with `--dry-run` for all template operations
  - Full support for `[project.optional-dependencies]` in pyproject.toml
- **Migration from pip freeze to pyproject.toml**: Modern dependency management
  - pyproject.toml-based configuration instead of requirements.txt
  - Lock file support via uv.lock for reproducible builds
  - Optional dependency groups for dev tools, testing, etc.
- **Safety Features**:
  - Automatic backup creation in `sync` command (pyproject.toml.backup)
  - Automatic rollback on failure (uv lock/sync errors)
  - Preview changes before execution with `--dry-run`

### Changed

- **Breaking**: Replaced `uvup copy` with `uvup clone` for clearer semantics
- Environment structure now includes:
  - `pyproject.toml` - Project configuration and dependencies
  - `uv.lock` - Lock file for reproducible installs
  - `.venv/` - Virtual environment directory
  - `hello.py` - Optional demo file

### Removed

- `uvup copy` command (replaced by `uvup clone`)
- Direct pip freeze dependency management (replaced by pyproject.toml)

### Documentation

- Complete command reference in docs/COMMANDS.md (570 lines)
- Real-world usage scenarios in docs/USE_CASES.md (489 lines)
- Updated README with four-method architecture explanation
- Simplified Core Philosophy to concise bullet points
- Added "Documentation" section with quick help and complete docs links
- Updated TARGET.md with v0.2.0 status and v0.3.0 planning

### Technical

- Enhanced `commands/new.rs` with full template modification support
- Enhanced `commands/sync.rs` with backup and rollback mechanism
- Refactored `commands/copy.rs` to `commands/clone.rs`
- Added comprehensive dry-run implementation across all template commands
- Improved dependency filtering logic with PEP 508 specifier support

## [0.1.3] - 2025-11-16

### Added

- `uvup copy <source> <target>` - Copy an existing environment to a new one
- `--python <version>` flag for copy command to override Python version

### Changed

- Upgraded `ureq` dependency from v2.x to v3.0 for improved HTTP handling

### Technical

- Added `commands/copy.rs` module
- Enhanced CLI with Copy command variant

## [0.1.2] - 2025-11-16

### Added

- `uvup update` - Self-update command to install latest version from GitHub releases
- `uvup update --check` - Check for updates without installing

### Fixed

- Shell validation in deactivate command to prevent errors when called outside shell context

### Technical

- Added `commands/update.rs` module
- Added GitHub release checking and binary replacement logic

## [0.1.1] - 2025-11-15

### Fixed

- Installation script compatibility issues on macOS and Linux
- Build target for Linux changed from `unknown-linux-gnu` to `unknown-linux-musl` for better portability

### Technical

- Updated installation scripts (install.sh, install.ps1)
- Modified build configuration for Linux releases

## [0.1.0] - 2025-11-15

### Added

- Initial MVP release of uvup
- `uvup init` - Initialize shell integration for Bash, Zsh, Fish, and PowerShell
- `uvup create <name>` - Create virtual environments using uv
- `uvup list` - List all managed environments
- `uvup remove <name>` - Remove virtual environments
- `uvup activate <name>` - Activate environments (via shell hook)
- `uvup deactivate` - Deactivate current environment (via shell hook)
- Support for flexible argument order in `uvup create` (e.g., `--python` before or after name)
- Default Python version set to 3.12
- Centralized environment storage in `~/.uvup/`
- Cross-platform path handling using `dirs` crate
- Comprehensive error handling with user-friendly messages
- Unit tests for core functionality
- Integration tests for command workflows

### Documentation

- README with installation and usage instructions
- VSCode IDE integration guide
- Development workflow documentation

### Dependencies

- clap 4.5 - CLI framework
- dirs 6.0 - Cross-platform directory paths

[0.2.1]: https://github.com/KercyDing/uvup/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/KercyDing/uvup/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/KercyDing/uvup/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/KercyDing/uvup/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/KercyDing/uvup/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/KercyDing/uvup/releases/tag/v0.1.0
