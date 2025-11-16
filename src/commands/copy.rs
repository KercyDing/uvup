// Allow println! in this module as it's used for user-facing output
#![allow(clippy::print_stdout)]

use crate::env::paths::{get_env_path, get_envs_dir, validate_env_name};
use crate::error::{Result, UvupError};
use crate::utils::print_success;
use std::fs;
use std::path::Path;
use std::process::Command;

pub(crate) fn run(source: String, target: String) -> Result<()> {
    validate_env_name(&source)?;
    validate_env_name(&target)?;

    let source_path = get_env_path(&source)?;
    if !source_path.exists() {
        return Err(UvupError::EnvNotFound(source));
    }

    let target_path = get_env_path(&target)?;
    if target_path.exists() {
        return Err(UvupError::EnvAlreadyExists(target));
    }

    println!("Copying environment '{source}' to '{target}'...");

    // Export packages from source environment
    println!("Exporting packages from '{source}'...");
    let requirements = export_packages(&source_path)?;

    // Detect Python version from source environment
    let python_version = get_python_version(&source_path)?;

    // Create target environment
    println!("Creating environment '{target}' with Python {python_version}...");
    create_environment(&target, &python_version)?;

    // Sync packages to target environment
    if requirements.trim().is_empty() {
        println!("Source environment has no packages installed");
        print_success(&format!("Created empty environment '{target}'"));
    } else {
        println!("Installing packages...");
        sync_packages(&target_path, &requirements)?;
        print_success(&format!(
            "Successfully copied environment '{source}' to '{target}'"
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
