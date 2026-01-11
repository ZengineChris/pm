# pm - Project Manager CLI

A lightweight, fast CLI tool for managing development projects organized by git hosting provider.

## Features

- **Organize projects by hosting provider** - Automatically groups projects by github.com, gitlab.com, azure.com, etc.
- **Track project metadata** - Store project descriptions, repository URLs, and local paths
- **Git worktree support** - Special handling for projects using git worktrees
- **Multiple output formats** - View project lists in table, JSON, or YAML format
- **Git integration** - Clone, check status, and update projects directly
- **Navigation helpers** - Quickly navigate to project directories
- **Search and filter** - Find projects by name, description, hosting, or worktree status

## Installation

### Using Nix Flakes

```bash
# Add to your flake inputs
inputs.pm.url = "github:zengineChris/pm";

# Or run directly
nix run github:zengineChris/pm
```

### Using Cargo

```bash
cargo install --path .
```

### From Source

```bash
git clone https://github.com/zengineChris/pm.git
cd pm
cargo build --release
sudo cp target/release/pm /usr/local/bin/
```

## Quick Start

```bash
# Initialize configuration
pm init

# Create a new project
pm create my-project -r https://github.com/user/repo.git -d "My awesome project"

# Create and clone a project
pm create another-project -r https://github.com/user/another.git --clone

# Create a worktree project
pm create wt-project -r https://github.com/user/wt.git -w --clone

# List all projects
pm list

# List projects with JSON output
pm list -o json

# Navigate to a project (use with eval or $())
cd "$(pm navigate my-project)"

# Check git status of all projects
pm status

# Update all projects
pm update --all

# Search for projects
pm search "awesome"

# Delete a project
pm delete my-project
```

## Configuration

Projects are stored in `~/.config/pm/projects.toml`:

```toml
version = "1.0"

[settings]
default_output_format = "table"

[hostings.github]
base_path = "~/github.com"
url_pattern = "github.com"

[hostings.gitlab]
base_path = "~/gitlab.com"
url_pattern = "gitlab.com"

[hostings.azure]
base_path = "~/azure.com"
url_pattern = "azure.com"

[hostings.custom]
base_path = "~/git"
url_pattern = ""

[[projects]]
name = "pm"
description = "Project manager CLI"
repository_url = "https://github.com/zengineChris/pm.git"
is_worktree = false
hosting = "github"
local_path = "~/github.com/zengineChris/pm"
created_at = "2026-01-11T10:27:00Z"
last_updated = "2026-01-11T10:27:00Z"
```

## Shell Integration

Add this function to your `.bashrc` or `.zshrc` for easy navigation:

```bash
# Navigate to projects with pm
pcd() {
    local path
    path=$(pm navigate "$1" 2>/dev/null)
    if [ $? -eq 0 ]; then
        eval "$path"
    else
        echo "Project not found: $1"
        return 1
    fi
}
```

Then use:

```bash
pcd my-project  # Automatically cd to project directory
```

## Commands

### Project Management

- `pm create <NAME>` - Create a new project
  - `-r, --repo <URL>` - Repository URL
  - `-d, --description <DESC>` - Project description
  - `-w, --worktree` - Mark as worktree project
  - `-H, --hosting <HOST>` - Hosting service (github, gitlab, azure, custom)
  - `-p, --path <PATH>` - Custom local path
  - `--clone` - Clone repository after creating

- `pm list` - List all projects
  - `-H, --hosting <HOST>` - Filter by hosting
  - `-w, --worktree` - Show only worktrees
  - `--no-worktree` - Show only non-worktrees
  - `-s, --search <PATTERN>` - Filter by name/description
  - `-o, --output <FORMAT>` - Output format (json|table|yaml)

- `pm delete <NAME>` - Delete a project
  - `-f, --force` - Skip confirmation
  - `--delete-files` - Also delete local files
  - `--keep-files` - Keep local files (default)

- `pm edit <NAME>` - Edit project metadata
  - `-d, --description <DESC>` - Update description
  - `-r, --repo <URL>` - Update repository URL
  - `-w, --worktree` - Mark as worktree
  - `--no-worktree` - Mark as not worktree
  - `-n, --name <NEW_NAME>` - Rename project
  - `-p, --path <PATH>` - Update local path

- `pm search <PATTERN>` - Search projects
  - `-H, --hosting <HOST>` - Filter by hosting
  - `-w, --worktree` - Search only worktrees
  - `-i, --ignore-case` - Case-insensitive search

- `pm navigate <NAME>` - Get navigation command
  - Usage: `$(pm navigate project-name)` or with the `pcd` helper function

### Git Operations

- `pm status` - Show git status for projects
  - `-H, --hosting <HOST>` - Filter by hosting
  - `-n, --name <NAME>` - Check specific project
  - `--dirty` - Show only dirty projects

- `pm update` - Update/pull projects
  - `-H, --hosting <HOST>` - Update hosting group
  - `-n, --name <NAME>` - Update specific project
  - `--all` - Update all projects (default)
  - `--dry-run` - Show what would be updated

### Utilities

- `pm init` - Initialize pm configuration
  - `--force` - Overwrite existing config

- `pm completions <SHELL>` - Generate shell completions
  - Supports: bash, zsh, fish, powershell, elvish

## Worktree Projects

When creating a project with the `--worktree` flag, pm handles the directory structure specially:

- **Normal project**: `~/github.com/user/repo/` (repo cloned directly)
- **Worktree project**: `~/github.com/user/repo/main/` (repo cloned into default branch subdirectory)

This structure allows you to easily create additional worktrees as siblings:

```bash
# Create worktree project
pm create my-wt -r https://github.com/user/repo.git -w --clone
# Clones to: ~/github.com/user/repo/main/

# Later, add additional worktrees manually:
cd ~/github.com/user/repo
git worktree add feature-branch
# Creates: ~/github.com/user/repo/feature-branch/
```

## Output Formats

pm supports three output formats for list and search commands:

### Table (default)

```
┌────────────┬─────────────────┬─────────┬──────────┬─────────────────────────┐
│ Name       │ Description     │ Hosting │ Worktree │ Path                    │
├────────────┼─────────────────┼─────────┼──────────┼─────────────────────────┤
│ my-project │ My awesome proj │ github  │ no       │ ~/github.com/user/repo  │
└────────────┴─────────────────┴─────────┴──────────┴─────────────────────────┘
```

### JSON

```bash
pm list -o json
```

```json
[
  {
    "name": "my-project",
    "description": "My awesome project",
    "repository_url": "https://github.com/user/repo.git",
    "is_worktree": false,
    "hosting": "github",
    "local_path": "~/github.com/user/repo",
    "created_at": "2026-01-11T10:27:00Z",
    "last_updated": "2026-01-11T10:27:00Z"
  }
]
```

### YAML

```bash
pm list -o yaml
```

```yaml
- name: my-project
  description: My awesome project
  repository_url: https://github.com/user/repo.git
  is_worktree: false
  hosting: github
  local_path: ~/github.com/user/repo
  created_at: '2026-01-11T10:27:00Z'
  last_updated: '2026-01-11T10:27:00Z'
```

## Global Options

All commands support these global options:

- `-o, --output <FORMAT>` - Output format (json|table|yaml)
- `-c, --config <PATH>` - Custom config file path
- `-v, --verbose` - Verbose output
- `-q, --quiet` - Quiet mode
- `-h, --help` - Print help
- `-V, --version` - Print version

## Development

### Requirements

- Rust 1.70+
- libgit2
- OpenSSL
- pkg-config

Or simply use Nix:

```bash
nix develop
cargo build
cargo test
cargo clippy
```

### Project Structure

```
pm/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli.rs           # CLI definitions
│   ├── error.rs         # Error types
│   ├── config/          # Configuration management
│   ├── commands/        # Command implementations
│   ├── git/             # Git operations
│   ├── models/          # Business logic
│   └── output/          # Output formatters
├── Cargo.toml
├── flake.nix
├── CLAUDE.md
└── README.md
```

See [CLAUDE.md](CLAUDE.md) for detailed architecture information.

## License

MIT

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
