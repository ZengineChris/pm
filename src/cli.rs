use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "pm")]
#[command(version, about = "Project Manager - Manage your development projects", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output format
    #[arg(short, long, global = true, value_enum)]
    pub output: Option<OutputFormatArg>,

    /// Custom config file path
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Quiet mode
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project
    #[command(visible_aliases = &["c", "new"])]
    Create(CreateArgs),

    /// List all projects
    #[command(visible_aliases = &["ls", "l"])]
    List(ListArgs),

    /// Delete a project
    #[command(visible_aliases = &["rm", "remove"])]
    Delete(DeleteArgs),

    /// Edit project metadata
    #[command(visible_alias = "e")]
    Edit(EditArgs),

    /// Show git status for projects
    #[command(visible_alias = "st")]
    Status(StatusArgs),

    /// Update/pull projects
    #[command(visible_alias = "pull")]
    Update(UpdateArgs),

    /// Search projects
    #[command(visible_aliases = &["find", "f"])]
    Search(SearchArgs),

    /// Get navigation command for a project
    #[command(visible_aliases = &["cd", "go"])]
    Navigate(NavigateArgs),

    /// Initialize pm configuration
    Init(InitArgs),

    /// Generate shell completions
    Completions(CompletionsArgs),
}

#[derive(clap::Args)]
pub struct CreateArgs {
    /// Project name
    pub name: String,

    /// Project description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Repository URL
    #[arg(short, long)]
    pub repo: Option<String>,

    /// Mark as worktree
    #[arg(short, long)]
    pub worktree: bool,

    /// Hosting service
    #[arg(short = 'H', long)]
    pub hosting: Option<String>,

    /// Custom local path
    #[arg(short, long)]
    pub path: Option<String>,

    /// Clone repository after creation
    #[arg(long)]
    pub clone: bool,
}

#[derive(clap::Args)]
pub struct ListArgs {
    /// Filter by hosting
    #[arg(short = 'H', long)]
    pub hosting: Option<String>,

    /// Show only worktrees
    #[arg(short, long)]
    pub worktree: bool,

    /// Show only non-worktrees
    #[arg(long)]
    pub no_worktree: bool,

    /// Filter by name/description
    #[arg(short, long)]
    pub search: Option<String>,
}

#[derive(clap::Args)]
pub struct DeleteArgs {
    /// Project name
    pub name: String,

    /// Skip confirmation
    #[arg(short, long)]
    pub force: bool,

    /// Also delete local files
    #[arg(long)]
    pub delete_files: bool,

    /// Keep local files (default)
    #[arg(long)]
    pub keep_files: bool,
}

#[derive(clap::Args)]
pub struct EditArgs {
    /// Project name
    pub name: String,

    /// Update description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Update repository URL
    #[arg(short, long)]
    pub repo: Option<String>,

    /// Mark as worktree
    #[arg(short, long)]
    pub worktree: bool,

    /// Mark as not worktree
    #[arg(long)]
    pub no_worktree: bool,

    /// Rename project
    #[arg(short, long)]
    pub name_new: Option<String>,

    /// Update local path
    #[arg(short, long)]
    pub path: Option<String>,
}

#[derive(clap::Args)]
pub struct StatusArgs {
    /// Filter by hosting
    #[arg(short = 'H', long)]
    pub hosting: Option<String>,

    /// Check specific project
    #[arg(short, long)]
    pub name: Option<String>,

    /// Show all projects (default)
    #[arg(long)]
    pub all: bool,

    /// Show only dirty projects
    #[arg(long)]
    pub dirty: bool,
}

#[derive(clap::Args)]
pub struct UpdateArgs {
    /// Filter by hosting
    #[arg(short = 'H', long)]
    pub hosting: Option<String>,

    /// Update specific project
    #[arg(short, long)]
    pub name: Option<String>,

    /// Update all projects (default)
    #[arg(long)]
    pub all: bool,

    /// Show what would be updated
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(clap::Args)]
pub struct SearchArgs {
    /// Search pattern
    pub pattern: String,

    /// Filter by hosting
    #[arg(short = 'H', long)]
    pub hosting: Option<String>,

    /// Search only worktrees
    #[arg(short, long)]
    pub worktree: bool,

    /// Case-insensitive search
    #[arg(short, long)]
    pub ignore_case: bool,
}

#[derive(clap::Args)]
pub struct NavigateArgs {
    /// Project name
    pub name: String,
}

#[derive(clap::Args)]
pub struct InitArgs {
    /// Overwrite existing config
    #[arg(long)]
    pub force: bool,
}

#[derive(clap::Args)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: Shell,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormatArg {
    Table,
    Json,
    Yaml,
}

#[derive(Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}
