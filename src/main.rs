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
        Commands::Init {
            shell,
            raw,
            reverse,
            dry_run,
        } => commands::init::run(shell, raw, reverse, dry_run)?,
        Commands::Create { name, python } => {
            commands::create::run(name, python.as_deref())?;
        }
        Commands::List => commands::list::run()?,
        Commands::Delete { name } => commands::delete::run(name)?,
        Commands::Clone { source, target } => commands::clone::run(source, target)?,
        Commands::New {
            name,
            template,
            python,
            exclude,
            include,
            path,
            dry_run,
        } => commands::new::run(
            &name,
            template,
            python.as_deref(),
            exclude.as_deref(),
            include.as_deref(),
            path.as_deref(),
            dry_run,
        )?,
        Commands::Sync {
            template,
            python,
            exclude,
            include,
            dry_run,
        } => commands::sync::run(
            template,
            python.as_deref(),
            exclude.as_deref(),
            include.as_deref(),
            dry_run,
        )?,
        Commands::Update { check } => commands::update::run(check)?,
        Commands::Add { packages, group } => commands::add::run(&packages, group)?,
        Commands::Remove { packages, group } => commands::remove::run(&packages, group)?,
        Commands::Lock { upgrade } => commands::lock::run(upgrade)?,
        Commands::Tree { depth } => commands::tree::run(depth)?,
    }

    Ok(())
}
