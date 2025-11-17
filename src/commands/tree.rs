use crate::env::paths::get_env_path;
use crate::error::{Result, UvupError};
use std::env;
use std::process::Command;

pub(crate) fn run(depth: Option<usize>) -> Result<()> {
    let active_env = env::var("UVUP_ACTIVE_ENV").map_err(|_| UvupError::NoActiveEnvironment)?;

    let env_path = get_env_path(&active_env)?;

    if !env_path.exists() {
        return Err(UvupError::EnvNotFound(active_env));
    }

    let mut cmd = Command::new("uv");
    cmd.arg("--project").arg(&env_path).arg("tree");

    if let Some(d) = depth {
        cmd.arg("--depth").arg(d.to_string());
    }

    let status = cmd.status().map_err(|e| {
        UvupError::CommandExecutionFailed(format!("Failed to execute uv tree: {e}"))
    })?;

    if status.success() {
        Ok(())
    } else {
        Err(UvupError::CommandExecutionFailed(
            "uv tree command failed".to_string(),
        ))
    }
}
