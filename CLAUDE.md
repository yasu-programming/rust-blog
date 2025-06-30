# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Status

This is an empty Rust blog project repository that has been initialized but contains no actual Rust code yet. The repository currently only contains:
- A minimal README.md file with just the project name "rust-blog"
- Git initialization (clean working directory)

## Development Setup

### GitHub Codespaces

This project is configured for GitHub Codespaces development with a `.devcontainer` setup that includes:
- Rust development environment (latest stable)
- Essential VS Code extensions (rust-analyzer, LLDB debugger, etc.)
- GitHub CLI and Node.js for additional tooling
- **Claude Code CLI** - Automatically installed and configured

#### Using Claude Code in Codespaces

After the Codespace starts, Claude Code will be automatically installed. To use it:

1. Authenticate with your API key:
   ```bash
   claude-code auth
   ```

2. Start using Claude Code:
   ```bash
   claude-code
   ```

The setup script (`.devcontainer/setup-claude.sh`) handles the installation and PATH configuration automatically.

### Local Development

For local development, typical Rust development commands would be:

```bash
# Initialize a new Rust project (if not already done)
cargo init

# Build the project
cargo build

# Run the project
cargo run

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

## Next Steps

This repository appears to be in its initial state and will need:
1. Rust project initialization (`cargo init`)
2. Basic project structure setup
3. Dependencies definition in Cargo.toml
4. Source code implementation

The project name suggests it's intended to be a blog application built with Rust.