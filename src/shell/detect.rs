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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_type_equality() {
        assert_eq!(ShellType::Bash, ShellType::Bash);
        assert_eq!(ShellType::Zsh, ShellType::Zsh);
        assert_ne!(ShellType::Bash, ShellType::Zsh);
    }

    #[test]
    fn test_detect_shell_current() {
        let result = detect_shell();
        assert!(result.is_ok());

        let shell = result.unwrap();
        #[cfg(not(target_os = "windows"))]
        {
            assert!(matches!(
                shell,
                ShellType::Bash | ShellType::Zsh | ShellType::Fish
            ));
        }

        #[cfg(target_os = "windows")]
        {
            assert_eq!(shell, ShellType::PowerShell);
        }
    }
}
