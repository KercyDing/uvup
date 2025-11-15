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

        #[arg(long, help = "Python version (default: 3.12)")]
        python: Option<String>,
    },

    #[command(about = "List all environments")]
    List,

    #[command(about = "Remove an environment")]
    Remove {
        #[arg(help = "Name of the environment to remove")]
        name: String,
    },
}
