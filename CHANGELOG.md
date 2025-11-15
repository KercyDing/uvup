# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.1.0]: https://github.com/KercyDing/uvup/releases/tag/v0.1.0
