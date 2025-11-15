# Contributing to uvup

Thank you for your interest in contributing to uvup! We appreciate your help in making this project better.

## Getting Started

### Prerequisites

- **Rust**: uvup is written in Rust. Install it from [rustup.rs](https://rustup.rs/)
- **uv**: Required for testing. Install from [astral.sh/uv](https://github.com/astral-sh/uv)
- **Git**: For version control

### Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/KercyDing/uvup.git
   cd uvup
   ```
3. Install the project in development mode:
   ```bash
   cargo install --path .
   ```
4. Initialize shell integration for testing:
   ```bash
   eval "$(uvup init)"
   ```

## Development Workflow

### Making Changes

1. Create a new branch for your work:
   ```bash
   git checkout -b your-name/feature
   ```

2. Make your changes following our [coding guidelines](#code-style)

3. Test your changes:
   ```bash
   # Run unit tests
   cargo test

   # Run clippy for linting
   cargo clippy -- -D warnings

   # Check code formatting
   cargo fmt --check
   ```

4. Format your code:
   ```bash
   cargo fmt
   ```

5. Reinstall to test the binary:
   ```bash
   cargo install --path .
   ```

### Testing

uvup uses both unit tests and integration tests:

**Unit Tests**
- Located in the same files as the code (using `#[cfg(test)]` modules)
- Test individual functions and modules
- Run with `cargo test`

**Integration Tests**
- Located in `tests/` directory
- Test complete workflows
- Run with `cargo test --test integration_test`

**Manual Testing**
- Test all commands in your shell:
  ```bash
  uvup create test-env
  uvup list
  uvup activate test-env
  uvup deactivate
  uvup remove test-env
  ```
- Test on different shells (Bash, Zsh, Fish, PowerShell if possible)
- Test edge cases (special characters, spaces, etc.)

### Code Style

We follow standard Rust conventions:

- **Formatting**: Use `rustfmt` with default settings
- **Linting**: All code must pass `cargo clippy -- -D warnings`
- **Naming**: Follow Rust naming conventions
- **Comments**: Write comments in English only
- **Documentation**: Use rustdoc comments (`///`) for public APIs


### Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>: <description>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test additions or changes
- `refactor`: Code refactoring
- `chore`: Maintenance tasks
- `perf`: Performance improvements

**Examples**:
```
feat: add default environment support
fix: handle spaces in environment names correctly
docs: update installation instructions
test: add integration tests for Windows
```

A git commit-msg hook is provided to enforce this format.

### Pull Requests

1. Push your changes to your fork:
   ```bash
   git push origin your-name/feature
   ```

2. Open a pull request on GitHub

3. Ensure your PR:
   - Has a clear title and description
   - References any related issues
   - Passes all CI checks
   - Includes tests for new functionality
   - Updates documentation if needed

4. Respond to review feedback

## Project Structure

```
uvup/
├── src/
│   ├── main.rs              # Entry point, CLI parsing
│   ├── cli.rs               # Command-line argument definitions
│   ├── error.rs             # Error types
│   ├── commands/            # Command implementations
│   │   ├── init.rs
│   │   ├── create.rs
│   │   ├── list.rs
│   │   └── remove.rs
│   ├── shell/               # Shell integration
│   │   ├── detect.rs
│   │   ├── bash.rs
│   │   ├── fish.rs
│   │   └── powershell.rs
│   └── env/                 # Environment management
│       ├── paths.rs
│       └── manager.rs
├── tests/                   # Integration tests
├── ...
```

## Adding New Features

### Adding a New Command

1. Create a new file in `src/commands/` (e.g., `src/commands/your_command.rs`)
2. Implement the command logic with proper error handling
3. Export the module in `src/commands/mod.rs`
4. Add the command variant to `Commands` enum in `src/cli.rs`
5. Add routing in `src/main.rs`
6. Write unit tests in the same file
7. Add integration tests in `tests/`
8. Update documentation in README.md

### Adding Shell Support

1. Create a new hook template in `src/shell/` (e.g., `src/shell/nushell.rs`)
2. Add the shell type to `ShellType` enum in `src/shell/detect.rs`
3. Update the detection logic in `detect_shell()`
4. Export the module in `src/shell/mod.rs`
5. Update `src/commands/init.rs` to handle the new shell
6. Test thoroughly on the target shell
7. Update README.md with the new shell support

## Reporting Issues

When reporting bugs or requesting features:

1. Search existing issues first
2. Use the issue templates if available
3. Provide:
   - uvup version (`uvup --version`)
   - Operating system and shell
   - Steps to reproduce (for bugs)
   - Expected vs actual behavior
   - Error messages or logs

## Documentation

- **README.md**: User-facing documentation
- **CHANGELOG.md**: Version history
- **CONTRIBUTING.md**: 

When making changes, update relevant documentation.

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create a git tag: `git tag v0.x.0`
4. Push tag: `git push origin v0.x.0`
5. GitHub Actions will build and publish releases

## Community Guidelines

- Be respectful and constructive
- Follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)
- Help others when you can
- Ask questions when you need help

## Getting Help

- **Documentation**: Start with README.md
- **Issues**: Search or create an issue on GitHub
- **Discussions**: Use GitHub Discussions for general questions

## Language Policy

- **Code and comments**: English only
- **Commit messages**: English only
- **Documentation**: English (translations welcome)
- **Issue discussions**: English preferred, but we understand Chinese

## License

By contributing to uvup, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

Thank you for contributing to uvup!
