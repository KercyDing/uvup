// Allow println! in this file as it's used for user-facing output
#![allow(clippy::print_stdout)]

use crate::error::{Result, UvupError};
use crate::shell::detect::{ShellType, detect_shell};
use crate::shell::{bash, fish, powershell};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[cfg(not(target_os = "windows"))]
use std::collections::HashSet;

// For future CMD support
// #[cfg(target_os = "windows")]
// use winreg::enums::*;
// #[cfg(target_os = "windows")]
// use winreg::RegKey;

const INIT_MARKER_START: &str = "# uvup initialization";

pub(crate) fn run(shell: Option<String>, raw: bool, reverse: bool, dry_run: bool) -> Result<()> {
    // If --raw flag, print the script for current shell
    if raw {
        return print_current_shell_script();
    }

    // Determine which shells to initialize
    let shells_to_init = if let Some(shell_name) = shell {
        // Specific shell requested
        vec![parse_shell_type(&shell_name)?]
    } else {
        // No shell specified, initialize all detected shells
        detect_available_shells()
    };

    // Perform initialization or reverse
    for shell_type in shells_to_init {
        if reverse {
            uninitialize_shell(shell_type, dry_run)?;
        } else {
            initialize_shell(shell_type, dry_run)?;
        }
    }

    Ok(())
}

fn print_current_shell_script() -> Result<()> {
    let shell = detect_shell()?;
    let hook_script = match shell {
        ShellType::Bash | ShellType::Zsh => bash::BASH_HOOK,
        ShellType::Fish => fish::FISH_HOOK,
        ShellType::PowerShell => powershell::POWERSHELL_HOOK,
    };
    io::stdout().write_all(hook_script.as_bytes())?;
    Ok(())
}

fn parse_shell_type(name: &str) -> Result<ShellType> {
    match name.to_lowercase().as_str() {
        "powershell" | "pwsh" | "ps" => Ok(ShellType::PowerShell),
        "bash" | "git-bash" => Ok(ShellType::Bash),
        "zsh" => Ok(ShellType::Zsh),
        "fish" => Ok(ShellType::Fish),
        _ => Err(UvupError::PathError(format!("Unknown shell: {name}"))),
    }
}

fn detect_available_shells() -> Vec<ShellType> {
    let mut shells = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // On Windows, only auto-detect PowerShell and Bash
        // PowerShell is almost always available on Windows
        shells.push(ShellType::PowerShell);

        // Check for Git Bash
        if is_git_bash_installed() {
            shells.push(ShellType::Bash);
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On Unix-like systems, detect commonly used shells
        let mut detected = HashSet::new();

        // Check for bash
        if is_shell_available("bash") {
            detected.insert(ShellType::Bash);
        }

        // Check for zsh
        if is_shell_available("zsh") {
            detected.insert(ShellType::Zsh);
        }

        // Check for fish
        if is_shell_available("fish") {
            detected.insert(ShellType::Fish);
        }

        shells.extend(detected);
    }

    shells
}

#[cfg(target_os = "windows")]
fn is_git_bash_installed() -> bool {
    // Check common Git Bash installation paths
    let common_paths = [
        r"C:\Program Files\Git\bin\bash.exe",
        r"C:\Program Files (x86)\Git\bin\bash.exe",
    ];
    common_paths.iter().any(|p| PathBuf::from(p).exists())
}

#[cfg(not(target_os = "windows"))]
fn is_shell_available(shell_name: &str) -> bool {
    // Check if shell exists in common locations or PATH
    std::process::Command::new("which")
        .arg(shell_name)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn initialize_shell(shell: ShellType, dry_run: bool) -> Result<()> {
    match shell {
        ShellType::PowerShell => initialize_powershell(dry_run),
        ShellType::Bash | ShellType::Zsh => initialize_bash(dry_run),
        ShellType::Fish => initialize_fish_impl(dry_run),
    }
}

fn uninitialize_shell(shell: ShellType, dry_run: bool) -> Result<()> {
    match shell {
        ShellType::PowerShell => uninitialize_powershell(dry_run),
        ShellType::Bash | ShellType::Zsh => uninitialize_bash(dry_run),
        ShellType::Fish => uninitialize_fish_impl(dry_run),
    }
}

// PowerShell initialization
fn initialize_powershell(dry_run: bool) -> Result<()> {
    let profile_path = get_powershell_profile()?;

    if dry_run {
        println!("Would modify: {}", profile_path.display());
        println!("  Add uvup initialization to PowerShell profile");
        return Ok(());
    }

    // Create profile directory if it doesn't exist
    if let Some(parent) = profile_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Read existing content
    let content = if profile_path.exists() {
        fs::read_to_string(&profile_path)?
    } else {
        String::new()
    };

    // Check if already initialized
    if content.contains(INIT_MARKER_START) {
        println!("PowerShell already initialized (skipped)");
        return Ok(());
    }

    // Append initialization
    let init_code =
        format!("\n{INIT_MARKER_START}\nInvoke-Expression ((uvup init --raw) -join \"`n\")\n");

    let new_content = content + &init_code;
    fs::write(&profile_path, new_content)?;

    println!("✓ PowerShell initialized");
    println!("  Profile: {}", profile_path.display());
    println!("  Please restart your PowerShell session");

    Ok(())
}

fn uninitialize_powershell(dry_run: bool) -> Result<()> {
    let profile_path = get_powershell_profile()?;

    if !profile_path.exists() {
        println!("PowerShell profile not found (skipped)");
        return Ok(());
    }

    if dry_run {
        println!("Would modify: {}", profile_path.display());
        println!("  Remove uvup initialization from PowerShell profile");
        return Ok(());
    }

    let content = fs::read_to_string(&profile_path)?;

    // Remove initialization block
    let new_content = remove_init_block(&content);

    if new_content == content {
        println!("PowerShell not initialized (skipped)");
        return Ok(());
    }

    fs::write(&profile_path, new_content)?;
    println!("✓ Removed uvup from PowerShell");

    Ok(())
}

fn get_powershell_profile() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let home = env::var("USERPROFILE")
            .map_err(|_| UvupError::PathError("USERPROFILE not set".to_string()))?;

        // Try PowerShell Core first (7+)
        let core_profile = PathBuf::from(&home)
            .join("Documents")
            .join("PowerShell")
            .join("profile.ps1");

        if core_profile.exists() || is_pwsh_installed() {
            return Ok(core_profile);
        }

        // Fall back to Windows PowerShell
        Ok(PathBuf::from(&home)
            .join("Documents")
            .join("WindowsPowerShell")
            .join("profile.ps1"))
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(UvupError::PathError(
            "PowerShell profile detection not implemented for this OS".to_string(),
        ))
    }
}

#[cfg(target_os = "windows")]
fn is_pwsh_installed() -> bool {
    std::process::Command::new("pwsh")
        .arg("--version")
        .output()
        .is_ok()
}

// Bash initialization
fn initialize_bash(dry_run: bool) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        initialize_bash_windows(dry_run)
    }

    #[cfg(not(target_os = "windows"))]
    {
        initialize_bash_unix(dry_run)
    }
}

#[cfg(target_os = "windows")]
fn initialize_bash_windows(dry_run: bool) -> Result<()> {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|_| UvupError::PathError("HOME/USERPROFILE not set".to_string()))?;

    let home_path = PathBuf::from(home);
    let bashrc_path = home_path.join(".bashrc");
    let bash_profile_path = home_path.join(".bash_profile");

    if dry_run {
        if !bash_profile_path.exists() {
            println!("Would create: {}", bash_profile_path.display());
            println!("  Add .bashrc sourcing logic");
        }
        if !bashrc_path.exists() {
            println!("Would create: {}", bashrc_path.display());
        }
        println!("Would modify: {}", bashrc_path.display());
        println!("  Add uvup initialization to .bashrc");
        return Ok(());
    }

    // Step 1: Ensure .bash_profile exists and sources .bashrc
    let bash_profile_needs_update = if bash_profile_path.exists() {
        let profile_content = fs::read_to_string(&bash_profile_path)?;
        !profile_content.contains("source ~/.bashrc") && !profile_content.contains(". ~/.bashrc")
    } else {
        true
    };

    if bash_profile_needs_update {
        let bash_profile_content = if bash_profile_path.exists() {
            let existing = fs::read_to_string(&bash_profile_path)?;
            format!(
                "{existing}\n\n# Source .bashrc if it exists\nif [ -f ~/.bashrc ]; then\n    source ~/.bashrc\nfi\n"
            )
        } else {
            r"# Source .bashrc if it exists
if [ -f ~/.bashrc ]; then
    source ~/.bashrc
fi
"
            .to_string()
        };
        fs::write(&bash_profile_path, bash_profile_content)?;
        println!("✓ Created/Updated .bash_profile to source .bashrc");
    }

    // Step 2: Initialize .bashrc
    let bashrc_content = if bashrc_path.exists() {
        fs::read_to_string(&bashrc_path)?
    } else {
        String::new()
    };

    // Check if already initialized
    if bashrc_content.contains(INIT_MARKER_START) {
        println!("Bash already initialized (skipped)");
        return Ok(());
    }

    // Append initialization to .bashrc
    let init_code = format!("\n{INIT_MARKER_START}\neval \"$(uvup init --raw)\"\n");

    let new_content = bashrc_content + &init_code;
    fs::write(&bashrc_path, new_content)?;

    println!("✓ Bash initialized");
    println!("  Profile: {}", bash_profile_path.display());
    println!("  Config: {}", bashrc_path.display());
    println!("  Please restart your Bash session or run: source ~/.bash_profile");

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn initialize_bash_unix(dry_run: bool) -> Result<()> {
    let profile_path = get_bash_profile()?;

    if dry_run {
        println!("Would modify: {}", profile_path.display());
        println!("  Add uvup initialization to Bash profile");
        return Ok(());
    }

    let content = if profile_path.exists() {
        fs::read_to_string(&profile_path)?
    } else {
        String::new()
    };

    // Check if already initialized
    if content.contains(INIT_MARKER_START) {
        println!("Bash already initialized (skipped)");
        return Ok(());
    }

    // Append initialization
    let init_code = format!("\n{INIT_MARKER_START}\neval \"$(uvup init --raw)\"\n");

    let new_content = content + &init_code;
    fs::write(&profile_path, new_content)?;

    println!("✓ Bash initialized");
    println!("  Profile: {}", profile_path.display());
    println!(
        "  Please restart your Bash session or run: source {}",
        profile_path.display()
    );

    Ok(())
}

fn uninitialize_bash(dry_run: bool) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        uninitialize_bash_windows(dry_run)
    }

    #[cfg(not(target_os = "windows"))]
    {
        uninitialize_bash_unix(dry_run)
    }
}

#[cfg(target_os = "windows")]
fn uninitialize_bash_windows(dry_run: bool) -> Result<()> {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|_| UvupError::PathError("HOME/USERPROFILE not set".to_string()))?;

    let bashrc_path = PathBuf::from(home).join(".bashrc");

    if !bashrc_path.exists() {
        println!("Bash not initialized (skipped)");
        return Ok(());
    }

    if dry_run {
        println!("Would modify: {}", bashrc_path.display());
        println!("  Remove uvup initialization from .bashrc");
        return Ok(());
    }

    let content = fs::read_to_string(&bashrc_path)?;
    let new_content = remove_init_block(&content);

    if new_content == content {
        println!("Bash not initialized (skipped)");
        return Ok(());
    }

    fs::write(&bashrc_path, new_content)?;
    println!("✓ Removed uvup from Bash");

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn uninitialize_bash_unix(dry_run: bool) -> Result<()> {
    let profile_path = get_bash_profile()?;

    if !profile_path.exists() {
        println!("Bash profile not found (skipped)");
        return Ok(());
    }

    if dry_run {
        println!("Would modify: {}", profile_path.display());
        println!("  Remove uvup initialization from Bash profile");
        return Ok(());
    }

    let content = fs::read_to_string(&profile_path)?;
    let new_content = remove_init_block(&content);

    if new_content == content {
        println!("Bash not initialized (skipped)");
        return Ok(());
    }

    fs::write(&profile_path, new_content)?;
    println!("✓ Removed uvup from Bash");

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn get_bash_profile() -> Result<PathBuf> {
    let home = env::var("HOME").map_err(|_| UvupError::PathError("HOME not set".to_string()))?;

    let home_path = PathBuf::from(home);

    // On Unix, check in order: .bash_profile, .profile, .bashrc
    let bash_profile = home_path.join(".bash_profile");
    if bash_profile.exists() {
        return Ok(bash_profile);
    }

    let profile = home_path.join(".profile");
    if profile.exists() {
        return Ok(profile);
    }

    Ok(home_path.join(".bashrc"))
}

// Fish initialization
fn initialize_fish_impl(dry_run: bool) -> Result<()> {
    let config_path = get_fish_config()?;

    if dry_run {
        if !config_path.exists() {
            println!("Would create: {}", config_path.display());
        }
        println!("Would modify: {}", config_path.display());
        println!("  Add uvup initialization to Fish config");
        return Ok(());
    }

    // Create config directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Read existing content
    let content = if config_path.exists() {
        fs::read_to_string(&config_path)?
    } else {
        String::new()
    };

    // Check if already initialized
    if content.contains(INIT_MARKER_START) {
        println!("Fish already initialized (skipped)");
        return Ok(());
    }

    // Append initialization
    let init_code = format!("\n{INIT_MARKER_START}\nuvup init --raw | source\n");

    let new_content = content + &init_code;
    fs::write(&config_path, new_content)?;

    println!("✓ Fish initialized");
    println!("  Config: {}", config_path.display());
    println!(
        "  Please restart your Fish session or run: source {}",
        config_path.display()
    );

    Ok(())
}

fn uninitialize_fish_impl(dry_run: bool) -> Result<()> {
    let config_path = get_fish_config()?;

    if !config_path.exists() {
        println!("Fish config not found (skipped)");
        return Ok(());
    }

    if dry_run {
        println!("Would modify: {}", config_path.display());
        println!("  Remove uvup initialization from Fish config");
        return Ok(());
    }

    let content = fs::read_to_string(&config_path)?;
    let new_content = remove_init_block(&content);

    if new_content == content {
        println!("Fish not initialized (skipped)");
        return Ok(());
    }

    fs::write(&config_path, new_content)?;
    println!("✓ Removed uvup from Fish");

    Ok(())
}

fn get_fish_config() -> Result<PathBuf> {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|_| UvupError::PathError("HOME/USERPROFILE not set".to_string()))?;

    Ok(PathBuf::from(home)
        .join(".config")
        .join("fish")
        .join("config.fish"))
}

// Helper function to remove initialization block
fn remove_init_block(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut skip = false;

    for line in lines {
        if line.contains(INIT_MARKER_START) {
            skip = true;
            continue;
        }
        if skip && (line.is_empty() || line.trim().is_empty()) {
            skip = false;
            continue;
        }
        if skip && line.contains("uvup init") {
            continue;
        }
        if !skip {
            result.push(line);
        }
    }

    result.join("\n")
}
