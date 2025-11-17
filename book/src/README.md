# uvup

A conda-like environment manager for [uv](https://github.com/astral-sh/uv).

## What is uvup?

uvup is a companion tool for uv that provides:

- **Familiar Interface**: Conda-like commands (`create`, `activate`, `list`, etc.)
- **Centralized Management**: All environments in `~/.uvup/`
- **Template System**: Reusable project configurations
- **Package Management**: Integrated `add`, `remove`, `lock`, `tree` commands
- **Cross-Platform**: Works on Linux, macOS, and Windows

## Quick Example

```bash
# Create and activate environment
uvup create myproject
uvup activate myproject

# Add packages
uvup add requests numpy pandas

# Run your code
python script.py

# Deactivate
uvup deactivate
```

## Core Philosophy

- **Enhancement, not replacement**: Calls uv for actual work
- **Familiar interface**: Conda-like commands, seamless activation
- **Single binary**: Lightweight and cross-platform
- **Template-driven**: Reusable project configurations

## Why uvup?

If you're familiar with **conda** but want to use **uv** for its speed and modern Python tooling, uvup bridges the gap by providing:

1. **Global environment management** - Like conda's base environments
2. **Simple activation** - `uvup activate` instead of manual PATH manipulation
3. **Template-based workflows** - Share project configurations across teams
4. **Integrated package commands** - Manage packages from anywhere after activation

## Getting Started

Continue to the [Installation](./installation.md) guide to get started!
