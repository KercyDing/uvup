// Allow println! in this module as it's used for user-facing output
#![allow(clippy::print_stdout)]

use crate::env::paths::{get_env_path, get_envs_dir, validate_env_name};
use crate::error::{Result, UvupError};
use crate::utils::print_success;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn run(
    source: String,
    name: Option<String>,
    python: Option<&str>,
    exclude: Option<&[String]>,
    include: Option<&[String]>,
    local: bool,
) -> Result<()> {
    validate_env_name(&source)?;

    // Determine target path and name
    let (target_name, target_path) = if local {
        let current_dir = env::current_dir()
            .map_err(|e| UvupError::PathError(format!("Failed to get current directory: {e}")))?;
        let venv_path = current_dir.join(".venv");

        // Prompt user for confirmation
        print!("Create .venv in '{}'? [Y/n] ", current_dir.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(UvupError::IoError)?;

        let input = input.trim().to_lowercase();
        if !input.is_empty() && input != "y" && input != "yes" {
            println!("Cancelled.");
            return Ok(());
        }

        (".venv".to_string(), venv_path)
    } else {
        let target = name
            .ok_or_else(|| UvupError::InvalidInput("Must provide --name or --local".to_string()))?;
        validate_env_name(&target)?;
        let path = get_env_path(&target)?;
        (target, path)
    };

    let source_path = get_env_path(&source)?;
    if !source_path.exists() {
        return Err(UvupError::EnvNotFound(source));
    }

    if target_path.exists() {
        return Err(UvupError::EnvAlreadyExists(target_name.clone()));
    }

    println!("Copying environment '{source}' to '{target_name}'...");

    // Export packages from source environment
    println!("Exporting packages from '{source}'...");
    let mut requirements = export_packages(&source_path)?;

    // Apply filters if specified
    if exclude.is_some() || include.is_some() {
        requirements = filter_packages(&requirements, exclude, include);
    }

    // Determine Python version: use provided version or detect from source
    let python_version = if let Some(version) = python {
        version.to_string()
    } else {
        get_python_version(&source_path)?
    };

    // Create target environment
    println!("Creating environment '{target_name}' with Python {python_version}...");
    if local {
        create_project_environment(&target_path, &python_version)?;
    } else {
        create_environment(&target_name, &python_version)?;
    }

    // Sync packages to target environment
    if requirements.trim().is_empty() {
        println!("Source environment has no packages installed");
        print_success(&format!("Created empty environment '{target_name}'"));
    } else {
        println!("Installing packages...");
        sync_packages(&target_path, &requirements)?;
        print_success(&format!(
            "Successfully copied environment '{source}' to '{target_name}'"
        ));
    }

    Ok(())
}

/// Export packages from an environment using uv pip freeze
fn export_packages(env_path: &Path) -> Result<String> {
    let python_bin = if cfg!(windows) {
        env_path.join("Scripts").join("python.exe")
    } else {
        env_path.join("bin").join("python")
    };

    let output = Command::new("uv")
        .arg("pip")
        .arg("freeze")
        .arg("--python")
        .arg(&python_bin)
        .output()
        .map_err(|e| {
            UvupError::CommandExecutionFailed(format!("Failed to run uv pip freeze: {e}"))
        })?;

    if !output.status.success() {
        return Err(UvupError::CommandExecutionFailed(
            "Failed to export packages".to_string(),
        ));
    }

    String::from_utf8(output.stdout).map_err(|e| {
        UvupError::CommandExecutionFailed(format!("Invalid UTF-8 in package list: {e}"))
    })
}

/// Get Python version from pyvenv.cfg
fn get_python_version(env_path: &Path) -> Result<String> {
    let cfg_path = env_path.join("pyvenv.cfg");
    let cfg_content = fs::read_to_string(&cfg_path)
        .map_err(|e| UvupError::PathError(format!("Failed to read pyvenv.cfg: {e}")))?;

    // Parse version_info line (e.g., "version_info = 3.12.11")
    for line in cfg_content.lines() {
        if let Some(version) = line.strip_prefix("version_info = ") {
            let parts: Vec<&str> = version.split('.').collect();
            if parts.len() >= 2 {
                return Ok(format!("{}.{}", parts[0], parts[1]));
            }
        }
    }

    // Fallback to default version
    Ok("3.12".to_string())
}

/// Create a new environment using uv venv
fn create_environment(name: &str, python_version: &str) -> Result<()> {
    let env_path = get_env_path(name)?;
    let envs_dir = get_envs_dir()?;
    fs::create_dir_all(&envs_dir)?;

    let status = Command::new("uv")
        .arg("venv")
        .arg(&env_path)
        .arg("--python")
        .arg(python_version)
        .status()
        .map_err(|e| UvupError::CommandExecutionFailed(format!("Failed to execute uv: {e}")))?;

    if !status.success() {
        return Err(UvupError::CommandExecutionFailed(format!(
            "Failed to create environment '{name}'"
        )));
    }

    Ok(())
}

/// Create a project .venv environment using uv venv
fn create_project_environment(venv_path: &PathBuf, python_version: &str) -> Result<()> {
    let status = Command::new("uv")
        .arg("venv")
        .arg(venv_path)
        .arg("--python")
        .arg(python_version)
        .status()
        .map_err(|e| UvupError::CommandExecutionFailed(format!("Failed to execute uv: {e}")))?;

    if !status.success() {
        return Err(UvupError::CommandExecutionFailed(
            "Failed to create project environment".to_string(),
        ));
    }

    Ok(())
}

/// Filter packages based on exclude/include patterns
fn filter_packages(
    requirements: &str,
    exclude: Option<&[String]>,
    include: Option<&[String]>,
) -> String {
    let lines: Vec<&str> = requirements.lines().collect();

    let filtered: Vec<&str> = lines
        .into_iter()
        .filter(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return true;
            }

            // Extract package name (before ==, >=, <=, etc.)
            let package_name = line
                .split(&['=', '>', '<', '~', '!'][..])
                .next()
                .unwrap_or(line)
                .trim()
                .to_lowercase();

            // If include list is specified, only keep packages in the list
            if let Some(include_list) = include {
                let included = include_list
                    .iter()
                    .any(|inc| package_name.contains(&inc.to_lowercase()));
                if !included {
                    return false;
                }
            }

            // If exclude list is specified, remove matching packages
            if let Some(exclude_list) = exclude {
                let excluded = exclude_list
                    .iter()
                    .any(|exc| package_name.contains(&exc.to_lowercase()));
                if excluded {
                    println!("  Excluding: {package_name}");
                    return false;
                }
            }

            true
        })
        .collect();

    filtered.join("\n")
}

/// Sync packages to target environment using uv pip sync
fn sync_packages(env_path: &Path, requirements: &str) -> Result<()> {
    // Create temporary requirements file
    let temp_file = tempfile::NamedTempFile::new().map_err(|e| {
        UvupError::CommandExecutionFailed(format!("Failed to create temp file: {e}"))
    })?;

    fs::write(&temp_file, requirements).map_err(|e| {
        UvupError::CommandExecutionFailed(format!("Failed to write requirements: {e}"))
    })?;

    let python_bin = if cfg!(windows) {
        env_path.join("Scripts").join("python.exe")
    } else {
        env_path.join("bin").join("python")
    };

    // Use uv pip sync for precise environment reproduction
    let status = Command::new("uv")
        .arg("pip")
        .arg("sync")
        .arg("--python")
        .arg(&python_bin)
        .arg(temp_file.path())
        .status()
        .map_err(|e| {
            UvupError::CommandExecutionFailed(format!("Failed to execute uv pip sync: {e}"))
        })?;

    if !status.success() {
        return Err(UvupError::CommandExecutionFailed(
            "Failed to install packages".to_string(),
        ));
    }

    Ok(())
}
