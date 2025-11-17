use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "uvup")]
#[command(about = "A conda-like environment manager for uv", long_about = None)]
#[command(version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    #[command(about = "Initialize uvup shell integration")]
    Init,

    #[command(about = "Create a new virtual environment")]
    Create {
        #[arg(help = "Name of the environment")]
        name: String,

        #[arg(short, long, global = true, help = "Python version (default: 3.12)")]
        python: Option<String>,
    },

    #[command(about = "List all environments")]
    List,

    #[command(about = "Remove an environment")]
    Remove {
        #[arg(help = "Name of the environment to remove")]
        name: String,
    },

    #[command(about = "Clone an environment (exact 1:1 copy)")]
    Clone {
        #[arg(help = "Source environment name")]
        source: String,

        #[arg(help = "Target environment name")]
        target: String,
    },

    #[command(about = "Create a new project from a template")]
    New {
        #[arg(help = "Project name")]
        name: String,

        #[arg(long, help = "Template environment name")]
        template: String,

        #[arg(short, long, help = "Python version (override template version)")]
        python: Option<String>,

        #[arg(
            long,
            value_delimiter = ',',
            help = "Exclude packages (comma-separated)"
        )]
        exclude: Option<Vec<String>>,

        #[arg(
            long,
            value_delimiter = ',',
            help = "Include only these packages (comma-separated)"
        )]
        include: Option<Vec<String>>,

        #[arg(long, help = "Directory to create project in (default: current dir)")]
        path: Option<String>,

        #[arg(long, help = "Preview changes without creating")]
        dry_run: bool,
    },

    #[command(about = "Sync current project with a template")]
    Sync {
        #[arg(long, help = "Template environment name")]
        template: String,

        #[arg(short, long, help = "Python version (override current version)")]
        python: Option<String>,

        #[arg(
            long,
            value_delimiter = ',',
            help = "Exclude packages (comma-separated)"
        )]
        exclude: Option<Vec<String>>,

        #[arg(
            long,
            value_delimiter = ',',
            help = "Include only these packages (comma-separated)"
        )]
        include: Option<Vec<String>>,

        #[arg(long, help = "Preview changes without syncing")]
        dry_run: bool,
    },

    #[command(about = "Update uvup to the latest version")]
    Update {
        #[arg(short, long, help = "Only check for updates without installing")]
        check: bool,
    },
}
