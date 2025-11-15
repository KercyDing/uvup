use crate::error::{Result, UvupError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

pub(crate) fn detect_shell() -> Result<ShellType> {
    if cfg!(target_os = "windows") {
        return Ok(ShellType::PowerShell);
    }

    if let Ok(shell_path) = std::env::var("SHELL") {
        if shell_path.contains("zsh") {
            return Ok(ShellType::Zsh);
        } else if shell_path.contains("bash") {
            return Ok(ShellType::Bash);
        } else if shell_path.contains("fish") {
            return Ok(ShellType::Fish);
        }
    }

    Err(UvupError::ShellDetectionFailed)
}
