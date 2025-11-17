// Allow println! in this module as it's used for user-facing output
#![allow(clippy::print_stdout)]

use crate::env::paths::{get_env_path, validate_env_name};
use crate::error::{Result, UvupError};
use crate::utils::print_success;
use std::fs;
use std::process::Command;

/// Clone an environment (exact 1:1 copy without modifications)
pub(crate) fn run(source: String, target: String) -> Result<()> {
    // Validate names
    validate_env_name(&source)?;
    validate_env_name(&target)?;

    // Check source exists
    let source_path = get_env_path(&source)?;
    if !source_path.exists() {
        return Err(UvupError::EnvNotFound(source));
    }

    // Check target doesn't exist
    let target_path = get_env_path(&target)?;
    if target_path.exists() {
        return Err(UvupError::EnvAlreadyExists(target));
    }

    println!("Cloning environment '{source}' to '{target}'...");

    // Create target directory
    fs::create_dir_all(&target_path)?;

    // Copy pyproject.toml
    let source_toml = source_path.join("pyproject.toml");
    let target_toml = target_path.join("pyproject.toml");
    fs::copy(&source_toml, &target_toml)
        .map_err(|e| UvupError::PathError(format!("Failed to copy pyproject.toml: {e}")))?;

    // Copy hello.py if exists
    let source_hello = source_path.join("hello.py");
    if source_hello.exists() {
        let target_hello = target_path.join("hello.py");
        fs::copy(&source_hello, &target_hello)
            .map_err(|e| UvupError::PathError(format!("Failed to copy hello.py: {e}")))?;
    }

    // Copy uv.lock if exists
    let source_lock = source_path.join("uv.lock");
    if source_lock.exists() {
        let target_lock = target_path.join("uv.lock");
        fs::copy(&source_lock, &target_lock)
            .map_err(|e| UvupError::PathError(format!("Failed to copy uv.lock: {e}")))?;
    }

    // Create venv
    println!("Creating virtual environment...");
    let venv_status = Command::new("uv")
        .arg("venv")
        .current_dir(&target_path)
        .status()
        .map_err(|e| {
            UvupError::CommandExecutionFailed(format!("Failed to execute uv venv: {e}"))
        })?;

    if !venv_status.success() {
        let _ = fs::remove_dir_all(&target_path);
        return Err(UvupError::CommandExecutionFailed(
            "Failed to create virtual environment".to_string(),
        ));
    }

    // Sync packages (use existing lock file if available)
    println!("Installing packages...");
    let sync_status = Command::new("uv")
        .arg("sync")
        .current_dir(&target_path)
        .status()
        .map_err(|e| {
            UvupError::CommandExecutionFailed(format!("Failed to execute uv sync: {e}"))
        })?;

    if !sync_status.success() {
        let _ = fs::remove_dir_all(&target_path);
        return Err(UvupError::CommandExecutionFailed(
            "Failed to install packages".to_string(),
        ));
    }

    print_success(&format!(
        "Successfully cloned environment '{source}' to '{target}'"
    ));

    Ok(())
}
