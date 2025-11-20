use crate::error::{Result, UvupError};
use std::io::{self, Write};
use std::process::Command;

pub(crate) fn print_success(message: &str) {
    let _ = writeln!(io::stdout(), "{message}");
}

pub(crate) fn print_info(message: &str) {
    let _ = writeln!(io::stdout(), "{message}");
}

pub(crate) fn verify_uv_installed() -> Result<()> {
    Command::new("uv")
        .arg("--version")
        .output()
        .map_err(|_| UvupError::UvNotFound)?;
    Ok(())
}
