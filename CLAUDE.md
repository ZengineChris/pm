# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`pm` (Project Manager) is a Rust-based CLI tool for managing development projects organized by git hosting provider. Projects are tracked in `~/.config/pm/projects.toml` with automatic directory organization by hosting (github.com, gitlab.com, azure.com, etc.).

## Development Commands

### Using Nix (Recommended)

```bash
# Enter development shell
nix develop

# Build the project
cargo build

# Run the CLI
cargo run -- <command>

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

### Without Nix

Ensure you have:
- Rust 1.70+ with cargo
- libgit2 development libraries
- OpenSSL development libraries
- pkg-config

Then use standard cargo commands.

## Architecture

### High-Level Structure

```
pm/
├── src/
│   ├── main.rs              # Entry point, CLI routing
│   ├── cli.rs               # Clap command definitions
│   ├── error.rs             # Error types (thiserror)
│   ├── config/              # Configuration management
│   │   ├── schema.rs        # Config & Project structs
│   │   ├── loader.rs        # TOML load/save
│   │   └── paths.rs         # Path resolution
│   ├── models/              # Business logic
│   │   └── project.rs       # Project path computation
│   ├── commands/            # Command implementations
│   │   ├── create.rs        # Create with optional --clone
│   │   ├── list.rs          # List with filtering
│   │   ├── delete.rs        # Delete project
│   │   ├── edit.rs          # Edit metadata
│   │   ├── status.rs        # Git status checking
│   │   ├── update.rs        # Bulk pull/update
│   │   ├── navigate.rs      # Print cd command
│   │   ├── search.rs        # Search/filter
│   │   └── init.rs          # Initialize config
│   ├── git/                 # Git operations
│   │   ├── clone.rs         # Clone with worktree support
│   │   ├── status.rs        # Status checking
│   │   └── update.rs        # Pull/fetch
│   └── output/              # Output formatters
│       ├── formatter.rs     # Formatter trait
│       ├── json.rs          # JSON output
│       ├── table.rs         # Table output
│       └── yaml.rs          # YAML output
└── Cargo.toml
```

### Key Design Decisions

1. **Configuration**: Single TOML file at `~/.config/pm/projects.toml` containing all project metadata
2. **Project Organization**: Projects grouped by hosting provider with automatic path computation from repository URLs
3. **Worktree Support**: When `is_worktree = true`, repositories are cloned into a subdirectory named after the default branch (e.g., `~/github.com/user/repo/main/`)
4. **Git Operations**: Uses `git2` (libgit2) for all git operations
5. **Output Formats**: Supports table, JSON, and YAML output via trait-based formatter pattern

### Configuration Schema

Projects are stored in `~/.config/pm/projects.toml`:

```toml
version = "1.0"

[settings]
default_output_format = "table"

[hostings.github]
base_path = "~/github.com"
url_pattern = "github.com"

[[projects]]
name = "my-project"
description = "A sample project"
repository_url = "https://github.com/user/repo.git"
is_worktree = false
hosting = "github"
local_path = "~/github.com/user/repo"
created_at = "2026-01-11T10:27:00Z"
last_updated = "2026-01-11T10:27:00Z"
```

### Entry Points

- `main.rs`: Parses CLI arguments and routes to command handlers
- Command handlers in `src/commands/`: Each command has its own module with an `execute()` function
- Config operations in `src/config/loader.rs`: `Config::load()`, `Config::save()`, `Config::init()`

### Error Handling

- Uses `thiserror` for typed errors (`ConfigError`, `ProjectError`, `PmError`)
- Uses `anyhow` in command handlers for flexibility
- All errors converted to human-readable messages before display

## Common Development Tasks

### Adding a New Command

1. Define the command in `src/cli.rs` (add to `Commands` enum and create args struct)
2. Create command module in `src/commands/<name>.rs`
3. Implement `execute()` function
4. Add module to `src/commands/mod.rs`
5. Add command routing in `main.rs`

### Adding a New Hosting Provider

Modify the default configuration in `src/config/schema.rs` to add the hosting provider to the default `hostings` HashMap.

### Modifying Project Schema

1. Update `Project` struct in `src/config/schema.rs`
2. Update serialization/deserialization
3. Handle migration if needed (version field in config)

## Testing

Run tests with:
```bash
cargo test
```

Integration tests are in `tests/` directory.

## Building for Release

```bash
cargo build --release
# Binary will be at target/release/pm
```

Or with Nix:
```bash
nix build
# Binary will be in result/bin/pm
```
