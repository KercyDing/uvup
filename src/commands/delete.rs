use crate::env::paths::{get_env_path, validate_env_name};
use crate::error::{Result, UvupError};
use crate::utils::print_success;
use std::fs;

pub(crate) fn run(name: String) -> Result<()> {
    validate_env_name(&name)?;

    let env_path = get_env_path(&name)?;

    if !env_path.exists() {
        return Err(UvupError::EnvNotFound(name));
    }

    fs::remove_dir_all(&env_path)?;

    print_success(&format!("Environment '{name}' removed"));
    Ok(())
}
