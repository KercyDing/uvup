use std::fmt;
use std::io;

#[derive(Debug)]
pub(crate) enum UvupError {
    UvNotFound,
    EnvAlreadyExists(String),
    EnvNotFound(String),
    InvalidEnvName(String),
    ShellDetectionFailed,
    IoError(io::Error),
    PathError(String),
    CommandExecutionFailed(String),
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
