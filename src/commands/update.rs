// Allow println! in this module as it's used for user-facing output
#![allow(clippy::print_stdout)]

use crate::error::{Result, UvupError};
use std::env;
use std::fs;
use std::io::Read;

const GITHUB_API_RELEASES: &str = "https://api.github.com/repos/KercyDing/uvup/releases/latest";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Check for updates without installing
pub(crate) fn check() -> Result<()> {
    println!("Current version: {CURRENT_VERSION}");
    println!("Checking for updates...");

    let latest_version = fetch_latest_version()?;

    if latest_version == CURRENT_VERSION {
        println!("You are already on the latest version!");
        return Ok(());
    }

    println!("New version available: {latest_version}");
    println!("Run 'uvup update' to install the latest version");

    Ok(())
}

/// Update uvup to the latest version
pub(crate) fn run(check_only: bool) -> Result<()> {
    if check_only {
        return check();
    }

    println!("Current version: {CURRENT_VERSION}");
    println!("Checking for updates...");

    let latest_version = fetch_latest_version()?;

    if latest_version == CURRENT_VERSION {
        println!("You are already on the latest version!");
        return Ok(());
    }

    println!("New version available: {latest_version}");
    println!("Downloading uvup {latest_version}...");

    let download_url = get_download_url(&latest_version)?;
    let binary_data = download_binary(&download_url)?;

    println!("Installing update...");
    replace_binary(&binary_data)?;

    println!("Successfully updated to version {latest_version}!");
    println!("Please restart your shell to use the new version.");

    Ok(())
}

/// Fetch the latest version from GitHub releases
fn fetch_latest_version() -> Result<String> {
    let mut response = ureq::get(GITHUB_API_RELEASES)
        .header("User-Agent", &format!("uvup/{CURRENT_VERSION}"))
        .call()
        .map_err(|e| UvupError::UpdateFailed(format!("Failed to fetch release info: {e}")))?;

    let json: serde_json::Value = response
        .body_mut()
        .read_json()
        .map_err(|e| UvupError::UpdateFailed(format!("Failed to parse release info: {e}")))?;

    let tag_name = json["tag_name"]
        .as_str()
        .ok_or_else(|| UvupError::UpdateFailed("Invalid release info".to_string()))?;

    // Remove 'v' prefix if present
    let version = tag_name.strip_prefix('v').unwrap_or(tag_name);

    Ok(version.to_string())
}

/// Get the download URL for the current platform
fn get_download_url(version: &str) -> Result<String> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    let filename = match (os, arch) {
        ("linux", "x86_64") => "uvup-linux-x86_64",
        ("linux", "aarch64") => "uvup-linux-arm64",
        ("macos", "x86_64") => "uvup-macos-x86_64",
        ("macos", "aarch64") => "uvup-macos-arm64",
        ("windows", "x86_64") => "uvup-windows-x86_64.exe",
        ("windows", "aarch64") => "uvup-windows-arm64.exe",
        _ => {
            return Err(UvupError::UpdateFailed(format!(
                "Unsupported platform: {os}-{arch}"
            )));
        }
    };

    Ok(format!(
        "https://github.com/KercyDing/uvup/releases/download/v{version}/{filename}"
    ))
}

/// Download the binary from the given URL
fn download_binary(url: &str) -> Result<Vec<u8>> {
    let mut response = ureq::get(url)
        .header("User-Agent", &format!("uvup/{CURRENT_VERSION}"))
        .call()
        .map_err(|e| UvupError::UpdateFailed(format!("Failed to download update: {e}")))?;

    let mut data = Vec::new();
    response
        .body_mut()
        .as_reader()
        .read_to_end(&mut data)
        .map_err(|e| UvupError::UpdateFailed(format!("Failed to read download: {e}")))?;

    Ok(data)
}

/// Replace the current binary with the new one
fn replace_binary(data: &[u8]) -> Result<()> {
    let current_exe = env::current_exe()
        .map_err(|e| UvupError::UpdateFailed(format!("Failed to get current executable: {e}")))?;

    // Create a temporary file next to the current executable
    let temp_path = current_exe.with_extension("new");

    fs::write(&temp_path, data)
        .map_err(|e| UvupError::UpdateFailed(format!("Failed to write new binary: {e}")))?;

    // Make the new binary executable (Unix-like systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&temp_path)
            .map_err(|e| UvupError::UpdateFailed(format!("Failed to get file permissions: {e}")))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&temp_path, perms)
            .map_err(|e| UvupError::UpdateFailed(format!("Failed to set permissions: {e}")))?;
    }

    // Replace the old binary with the new one
    self_replace::self_replace(&temp_path)
        .map_err(|e| UvupError::UpdateFailed(format!("Failed to replace binary: {e}")))?;

    Ok(())
}
