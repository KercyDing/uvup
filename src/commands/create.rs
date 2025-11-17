use crate::env::paths::{get_env_path, get_envs_dir, validate_env_name};
use crate::error::{Result, UvupError};
use crate::utils::print_success;
use std::fs;
use std::process::Command;

const DEFAULT_PYTHON_VERSION: &str = "3.12";

pub(crate) fn run(name: String, python_version: Option<&str>) -> Result<()> {
    validate_env_name(&name)?;

    let env_path = get_env_path(&name)?;

    if env_path.exists() {
        return Err(UvupError::EnvAlreadyExists(name));
    }

    let envs_dir = get_envs_dir()?;
    fs::create_dir_all(&envs_dir)?;

    verify_uv_installed()?;

    let py_version = python_version.unwrap_or(DEFAULT_PYTHON_VERSION);

    // Create project directory
    fs::create_dir_all(&env_path)?;

    // Initialize uv project
    let init_status = Command::new("uv")
        .arg("init")
        .arg("--no-readme")
        .arg("--python")
        .arg(py_version)
        .current_dir(&env_path)
        .status()
        .map_err(|e| {
            UvupError::CommandExecutionFailed(format!("Failed to execute uv init: {e}"))
        })?;

    if !init_status.success() {
        // Clean up on failure
        let _ = fs::remove_dir_all(&env_path);
        return Err(UvupError::CommandExecutionFailed(format!(
            "Failed to initialize project for environment '{name}'"
        )));
    }

    // Create virtual environment
    let venv_status = Command::new("uv")
        .arg("venv")
        .current_dir(&env_path)
        .status()
        .map_err(|e| {
            UvupError::CommandExecutionFailed(format!("Failed to execute uv venv: {e}"))
        })?;

    if venv_status.success() {
        print_success(&format!("Environment '{name}' created successfully"));
        Ok(())
    } else {
        // Clean up on failure
        let _ = fs::remove_dir_all(&env_path);
        Err(UvupError::CommandExecutionFailed(format!(
            "Failed to create environment '{name}'"
        )))
    }
}

fn verify_uv_installed() -> Result<()> {
    Command::new("uv")
        .arg("--version")
        .output()
        .map_err(|_| UvupError::UvNotFound)?;
    Ok(())
}
