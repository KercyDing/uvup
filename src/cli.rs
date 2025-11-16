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

    #[command(about = "Update uvup to the latest version")]
    Update {
        #[arg(short, long, help = "Only check for updates without installing")]
        check: bool,
    },

    #[command(
        about = "Copy an environment to a new environment",
        override_usage = "uvup copy <SOURCE> [OPTIONS] --name <NAME>"
    )]
    Copy {
        #[arg(help = "Source environment name")]
        source: String,

        #[arg(
            short,
            long,
            help = "Target environment name (or use --local)",
            conflicts_with = "local"
        )]
        name: Option<String>,

        #[arg(short, long, help = "Python version for target environment (optional)")]
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

        #[arg(short, long, help = "Copy to .venv in current directory")]
        local: bool,

        #[arg(long, help = "Override existing pyproject.toml (creates backup)")]
        r#override: bool,

        #[arg(long, help = "Preview changes without applying them")]
        dry_run: bool,
    },
}
