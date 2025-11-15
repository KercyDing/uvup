use crate::error::{Result, UvupError};
use std::path::PathBuf;

pub(crate) fn get_home_dir() -> Result<PathBuf> {
    dirs::home_dir()
        .ok_or_else(|| UvupError::PathError("Could not determine home directory".to_string()))
}

pub(crate) fn get_envs_dir() -> Result<PathBuf> {
    Ok(get_home_dir()?.join(".uvenvs"))
}

pub(crate) fn get_env_path(name: &str) -> Result<PathBuf> {
    validate_env_name(name)?;
    Ok(get_envs_dir()?.join(name))
}

pub(crate) fn validate_env_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(UvupError::InvalidEnvName(
            "Environment name cannot be empty".to_string(),
        ));
    }

    let valid_chars = name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_');

    if !valid_chars {
        return Err(UvupError::InvalidEnvName(name.to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_env_name() {
        assert!(validate_env_name("myproject").is_ok());
        assert!(validate_env_name("my-project").is_ok());
        assert!(validate_env_name("my_project").is_ok());
        assert!(validate_env_name("project123").is_ok());

        assert!(validate_env_name("").is_err());
        assert!(validate_env_name("my project").is_err());
        assert!(validate_env_name("my/project").is_err());
        assert!(validate_env_name("my.project").is_err());
    }
}
