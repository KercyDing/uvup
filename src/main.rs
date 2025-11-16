mod cli;
mod commands;
mod env;
mod error;
mod shell;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;
use std::io::{self, Write};

fn main() {
    if let Err(e) = run() {
        let _ = writeln!(io::stderr(), "{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run()?,
        Commands::Create { name, python } => {
            commands::create::run(name, python.as_deref())?;
        }
        Commands::List => commands::list::run()?,
        Commands::Remove { name } => commands::remove::run(name)?,
        Commands::Update { check } => commands::update::run(check)?,
    }

    Ok(())
}
