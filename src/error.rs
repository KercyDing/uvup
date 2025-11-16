use std::fmt;
use std::io;

#[derive(Debug)]
pub(crate) enum UvupError {
    UvNotFound,
    EnvAlreadyExists(String),
    EnvNotFound(String),
    InvalidEnvName(String),
    InvalidInput(String),
    ShellDetectionFailed,
    IoError(io::Error),
    PathError(String),
    CommandExecutionFailed(String),
    UpdateFailed(String),
}

impl fmt::Display for UvupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UvupError::UvNotFound => {
                writeln!(f, "Error: 'uv' command not found")?;
                write!(
                    f,
                    "Please install uv first: https://github.com/astral-sh/uv"
                )
            }
            UvupError::EnvAlreadyExists(name) => {
                write!(f, "Error: Environment '{name}' already exists")
            }
            UvupError::EnvNotFound(name) => {
                writeln!(f, "Error: Environment '{name}' not found")?;
                write!(f, "Tip: Use 'uvup list' to see all available environments")
            }
            UvupError::InvalidEnvName(name) => {
                writeln!(f, "Error: Invalid environment name '{name}'")?;
                write!(
                    f,
                    "Environment names must contain only alphanumeric characters, hyphens, and underscores"
                )
            }
            UvupError::InvalidInput(msg) => {
                write!(f, "Error: {msg}")
            }
            UvupError::ShellDetectionFailed => {
                writeln!(f, "Error: Could not detect your shell")?;
                write!(f, "Supported shells: bash, zsh, fish, powershell")
            }
            UvupError::IoError(err) => {
                write!(f, "IO Error: {err}")
            }
            UvupError::PathError(msg) => {
                write!(f, "Path Error: {msg}")
            }
            UvupError::CommandExecutionFailed(msg) => {
                write!(f, "Command execution failed: {msg}")
            }
            UvupError::UpdateFailed(msg) => {
                write!(f, "Update failed: {msg}")
            }
        }
    }
}

impl std::error::Error for UvupError {}

impl From<io::Error> for UvupError {
    fn from(err: io::Error) -> Self {
        UvupError::IoError(err)
    }
}

pub(crate) type Result<T> = std::result::Result<T, UvupError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = UvupError::UvNotFound;
        assert!(err.to_string().contains("'uv' command not found"));

        let err = UvupError::EnvAlreadyExists("test".to_string());
        assert!(err.to_string().contains("test"));
        assert!(err.to_string().contains("already exists"));

        let err = UvupError::EnvNotFound("myenv".to_string());
        assert!(err.to_string().contains("myenv"));
        assert!(err.to_string().contains("not found"));

        let err = UvupError::InvalidEnvName("bad-name".to_string());
        assert!(err.to_string().contains("Invalid environment name"));

        let err = UvupError::ShellDetectionFailed;
        assert!(err.to_string().contains("Could not detect your shell"));

        let err = UvupError::PathError("test error".to_string());
        assert!(err.to_string().contains("Path Error"));

        let err = UvupError::CommandExecutionFailed("test cmd".to_string());
        assert!(err.to_string().contains("Command execution failed"));

        let err = UvupError::UpdateFailed("network error".to_string());
        assert!(err.to_string().contains("Update failed"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let uvup_err: UvupError = io_err.into();

        match uvup_err {
            UvupError::IoError(_) => {}
            _ => panic!("Expected IoError variant"),
        }
    }

    #[test]
    fn test_result_type() {
        let ok_result: Result<i32> = Ok(42);
        assert!(ok_result.is_ok());
        if let Ok(value) = ok_result {
            assert_eq!(value, 42);
        }

        let err_result: Result<i32> = Err(UvupError::UvNotFound);
        assert!(err_result.is_err());
    }
}
