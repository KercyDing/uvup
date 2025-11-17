use crate::env::paths::get_envs_dir;
use crate::error::Result;
use crate::utils::print_info;
use std::fs;
use std::io::{self, Write};

pub(crate) fn run() -> Result<()> {
    let envs_dir = get_envs_dir()?;

    if !envs_dir.exists() {
        print_info("No environments found.");
        return Ok(());
    }

    let entries = fs::read_dir(&envs_dir)?;
    let mut env_names: Vec<String> = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    if is_valid_env(&path) {
                        env_names.push(name_str.to_string());
                    }
                }
            }
        }
    }

    if env_names.is_empty() {
        print_info("No environments found.");
    } else {
        env_names.sort();
        for name in env_names {
            writeln!(io::stdout(), "{name}")?;
        }
    }

    Ok(())
}

fn is_valid_env(path: &std::path::Path) -> bool {
    // Check for .venv subdirectory structure (new format)
    let venv_path = path.join(".venv");

    #[cfg(target_os = "windows")]
    {
        venv_path.join("Scripts").join("Activate.ps1").exists()
    }

    #[cfg(not(target_os = "windows"))]
    {
        venv_path.join("bin").join("activate").exists()
    }
}
