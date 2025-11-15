use crate::error::Result;
use crate::shell::detect::{ShellType, detect_shell};
use crate::shell::{bash, fish, powershell};
use std::io::{self, Write};

pub(crate) fn run() -> Result<()> {
    let shell = detect_shell()?;

    let hook_script = match shell {
        ShellType::Bash | ShellType::Zsh => bash::BASH_HOOK,
        ShellType::Fish => fish::FISH_HOOK,
        ShellType::PowerShell => powershell::POWERSHELL_HOOK,
    };

    io::stdout().write_all(hook_script.as_bytes())?;
    Ok(())
}
