// Allow println! in this module as it's used for user-facing output
#![allow(clippy::print_stdout)]

use crate::env::paths::{get_env_path, validate_env_name};
use crate::error::{Result, UvupError};
use crate::utils::print_success;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use toml_edit::{DocumentMut, Item, Value};

pub(crate) fn run(
    source: String,
    name: Option<String>,
    python: Option<&str>,
    exclude: Option<&[String]>,
    include: Option<&[String]>,
    local: bool,
    override_existing: bool,
) -> Result<()> {
    validate_env_name(&source)?;

    let source_path = get_env_path(&source)?;
    if !source_path.exists() {
        return Err(UvupError::EnvNotFound(source));
    }

    // Determine target configuration
    let target_config = determine_target_config(name, local, override_existing)?;
    check_target_exists(&target_config)?;

    println!(
        "Copying environment '{source}' to '{}'...",
        target_config.name
    );

    // Process pyproject.toml
    let mut doc = read_and_parse_toml(&source_path)?;

    if exclude.is_some() || include.is_some() {
        filter_dependencies(&mut doc, exclude, include)?;
    }

    let python_version = if let Some(version) = python {
        println!("Note: Switching Python version may cause package compatibility issues.");
        update_python_version(&mut doc, version)?;
        version.to_string()
    } else {
        get_python_version_from_toml(&doc)?
    };

    // Create environment
    println!(
        "Creating environment '{}' with Python {python_version}...",
        target_config.name
    );
    create_environment(&target_config, &source_path, &doc)?;

    // Sync packages
    println!("Installing packages...");
    sync_environment(&target_config)?;

    print_success(&format!(
        "Successfully copied environment '{source}' to '{}'",
        target_config.name
    ));

    Ok(())
}

struct TargetConfig {
    name: String,
    path: std::path::PathBuf,
    is_local: bool,
}

/// Determine target configuration based on user input
fn determine_target_config(
    name: Option<String>,
    local: bool,
    override_existing: bool,
) -> Result<TargetConfig> {
    if local {
        let current_dir = env::current_dir()
            .map_err(|e| UvupError::PathError(format!("Failed to get current directory: {e}")))?;

        let pyproject_path = current_dir.join("pyproject.toml");

        // Check if pyproject.toml exists
        if pyproject_path.exists() {
            if !override_existing {
                // No --override flag: tell user to add it
                return Err(UvupError::InvalidInput(
                    "Current directory already has pyproject.toml.\nUse --override to replace it (will create backup as pyproject.toml.backup).".to_string()
                ));
            }

            // With --override flag: ask for confirmation
            print!(
                "Replace existing pyproject.toml with environment configuration? [y/N] "
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(UvupError::IoError)?;

            let input = input.trim().to_lowercase();
            if input != "y" && input != "yes" {
                println!("Cancelled.");
                return Err(UvupError::InvalidInput("Cancelled by user".to_string()));
            }

            // Backup existing pyproject.toml
            let backup_path = current_dir.join("pyproject.toml.backup");
            fs::copy(&pyproject_path, &backup_path).map_err(|e| {
                UvupError::PathError(format!("Failed to create backup: {e}"))
            })?;
            println!("Backed up existing pyproject.toml to pyproject.toml.backup");
        } else {
            // No pyproject.toml: simple confirmation
            print!("Create .venv in '{}'? [Y/n] ", current_dir.display());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(UvupError::IoError)?;

            let input = input.trim().to_lowercase();
            if !input.is_empty() && input != "y" && input != "yes" {
                println!("Cancelled.");
                return Err(UvupError::InvalidInput("Cancelled by user".to_string()));
            }
        }

        Ok(TargetConfig {
            name: ".venv".to_string(),
            path: current_dir,
            is_local: true,
        })
    } else {
        let target = name
            .ok_or_else(|| UvupError::InvalidInput("Must provide --name or --local".to_string()))?;
        validate_env_name(&target)?;
        let path = get_env_path(&target)?;
        Ok(TargetConfig {
            name: target,
            path,
            is_local: false,
        })
    }
}

/// Check if target already exists
fn check_target_exists(config: &TargetConfig) -> Result<()> {
    if !config.is_local && config.path.exists() {
        return Err(UvupError::EnvAlreadyExists(config.name.clone()));
    }

    if config.is_local && config.path.join(".venv").exists() {
        return Err(UvupError::EnvAlreadyExists(".venv".to_string()));
    }

    Ok(())
}

/// Read and parse source pyproject.toml
fn read_and_parse_toml(source_path: &std::path::Path) -> Result<DocumentMut> {
    let source_toml_path = source_path.join("pyproject.toml");
    let toml_content = fs::read_to_string(&source_toml_path).map_err(|e| {
        UvupError::PathError(format!("Failed to read source pyproject.toml: {e}"))
    })?;

    toml_content.parse::<DocumentMut>().map_err(|e| {
        UvupError::CommandExecutionFailed(format!("Failed to parse pyproject.toml: {e}"))
    })
}

/// Create environment (local or project)
fn create_environment(
    config: &TargetConfig,
    source_path: &std::path::Path,
    doc: &DocumentMut,
) -> Result<()> {
    if config.is_local {
        create_local_environment(config, doc)
    } else {
        create_project_environment(config, source_path, doc)
    }
}

/// Create local .venv environment
fn create_local_environment(config: &TargetConfig, doc: &DocumentMut) -> Result<()> {
    fs::write(config.path.join("pyproject.toml"), doc.to_string()).map_err(|e| {
        UvupError::PathError(format!("Failed to write pyproject.toml: {e}"))
    })?;

    let venv_status = Command::new("uv")
        .arg("venv")
        .current_dir(&config.path)
        .status()
        .map_err(|e| UvupError::CommandExecutionFailed(format!("Failed to execute uv venv: {e}")))?;

    if !venv_status.success() {
        let _ = fs::remove_file(config.path.join("pyproject.toml"));
        return Err(UvupError::CommandExecutionFailed(
            "Failed to create virtual environment".to_string(),
        ));
    }

    Ok(())
}

/// Create project environment
fn create_project_environment(
    config: &TargetConfig,
    source_path: &std::path::Path,
    doc: &DocumentMut,
) -> Result<()> {
    fs::create_dir_all(&config.path)?;
    fs::write(config.path.join("pyproject.toml"), doc.to_string()).map_err(|e| {
        UvupError::PathError(format!("Failed to write pyproject.toml: {e}"))
    })?;

    // Copy hello.py if exists
    let source_hello = source_path.join("hello.py");
    if source_hello.exists() {
        fs::copy(&source_hello, config.path.join("hello.py"))
            .map_err(|e| UvupError::PathError(format!("Failed to copy hello.py: {e}")))?;
    }

    let venv_status = Command::new("uv")
        .arg("venv")
        .current_dir(&config.path)
        .status()
        .map_err(|e| UvupError::CommandExecutionFailed(format!("Failed to execute uv venv: {e}")))?;

    if !venv_status.success() {
        let _ = fs::remove_dir_all(&config.path);
        return Err(UvupError::CommandExecutionFailed(
            "Failed to create virtual environment".to_string(),
        ));
    }

    Ok(())
}

/// Lock and sync packages using explicit uv lock + uv sync
fn sync_environment(config: &TargetConfig) -> Result<()> {
    // Step 1: Explicitly lock dependencies
    println!("  Resolving and locking dependencies...");
    let lock_status = Command::new("uv")
        .arg("lock")
        .current_dir(&config.path)
        .status()
        .map_err(|e| UvupError::CommandExecutionFailed(format!("Failed to execute uv lock: {e}")))?;

    if !lock_status.success() {
        cleanup_failed_environment(config);
        return Err(UvupError::CommandExecutionFailed(
            "Failed to resolve and lock dependencies (possible version conflicts)".to_string(),
        ));
    }

    // Step 2: Explicitly sync environment with locked dependencies
    println!("  Installing locked packages...");
    let sync_status = Command::new("uv")
        .arg("sync")
        .current_dir(&config.path)
        .status()
        .map_err(|e| UvupError::CommandExecutionFailed(format!("Failed to execute uv sync: {e}")))?;

    if !sync_status.success() {
        cleanup_failed_environment(config);
        return Err(UvupError::CommandExecutionFailed(
            "Failed to install locked packages (possible network or permission issues)".to_string(),
        ));
    }

    Ok(())
}

/// Cleanup failed environment files/directories
fn cleanup_failed_environment(config: &TargetConfig) {
    if config.is_local {
        let _ = fs::remove_file(config.path.join("pyproject.toml"));
        let _ = fs::remove_file(config.path.join("uv.lock"));
        let _ = fs::remove_dir_all(config.path.join(".venv"));
    } else {
        let _ = fs::remove_dir_all(&config.path);
    }
}



/// Filter dependencies in pyproject.toml based on exclude/include patterns
fn filter_dependencies(
    doc: &mut DocumentMut,
    exclude: Option<&[String]>,
    include: Option<&[String]>,
) -> Result<()> {
    // Filter main dependencies
    if let Some(dependencies) = doc
        .get_mut("project")
        .and_then(|p| p.get_mut("dependencies"))
    {
        let deps_array = dependencies.as_array_mut().ok_or_else(|| {
            UvupError::CommandExecutionFailed("Invalid dependencies format".to_string())
        })?;

        let filtered = filter_dependency_array(deps_array, exclude, include);
        *deps_array = toml_edit::Array::from_iter(filtered);
    }

    // Filter optional-dependencies (if exists)
    if let Some(project) = doc.get_mut("project") {
        if let Some(optional_deps) = project.get_mut("optional-dependencies") {
            if let Some(optional_table) = optional_deps.as_table_mut() {
                let mut empty_groups = Vec::new();

                for (group_name, group_deps) in optional_table.iter_mut() {
                    if let Some(deps_array) = group_deps.as_array_mut() {
                        let filtered = filter_dependency_array(deps_array, exclude, include);

                        if filtered.is_empty() {
                            println!("  Note: Optional group '{group_name}' is now empty after filtering");
                            empty_groups.push(group_name.to_string());
                        } else {
                            *deps_array = toml_edit::Array::from_iter(filtered);
                        }
                    }
                }

                // Remove empty groups
                for group in empty_groups {
                    optional_table.remove(&group);
                }
            }
        }
    }

    Ok(())
}

/// Filter a single dependency array based on exclude/include patterns
fn filter_dependency_array(
    deps_array: &toml_edit::Array,
    exclude: Option<&[String]>,
    include: Option<&[String]>,
) -> Vec<toml_edit::Value> {
    let mut filtered_deps = Vec::new();

    for dep in deps_array {
        if let Some(dep_str) = dep.as_str() {
            let package_name = extract_package_name(dep_str);

            // Apply include filter first
            if let Some(include_list) = include {
                let included = include_list
                    .iter()
                    .any(|inc| package_name == inc.to_lowercase());
                if !included {
                    continue;
                }
            }

            // Apply exclude filter
            if let Some(exclude_list) = exclude {
                let excluded = exclude_list
                    .iter()
                    .any(|exc| package_name == exc.to_lowercase());
                if excluded {
                    println!("  Excluding: {package_name}");
                    continue;
                }
            }

            filtered_deps.push(dep.clone());
        }
    }

    filtered_deps
}

/// Extract core package name from dependency string
///
/// Handles various formats:
/// - "requests>=2.31.0" -> "requests"
/// - "requests[http3]>=2.0" -> "requests"
/// - "requests~=2.31.0" -> "requests"
/// - "my-package>=1.0" -> "my-package"
fn extract_package_name(dep_str: &str) -> String {
    // Find the first occurrence of version specifier or bracket
    let end_pos = dep_str
        .find(['=', '>', '<', '~', '!', '['])
        .unwrap_or(dep_str.len());

    dep_str[..end_pos].trim().to_lowercase()
}


/// Get Python version from pyproject.toml
fn get_python_version_from_toml(doc: &DocumentMut) -> Result<String> {
    let version_str = doc
        .get("project")
        .and_then(|p| p.get("requires-python"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            UvupError::CommandExecutionFailed(
                "No requires-python found in pyproject.toml".to_string(),
            )
        })?;

    // Parse version string like ">=3.12" to "3.12"
    let version = version_str
        .trim_start_matches(|c: char| !c.is_ascii_digit())
        .split('.')
        .take(2)
        .collect::<Vec<_>>()
        .join(".");

    Ok(version)
}

/// Update Python version in pyproject.toml
fn update_python_version(doc: &mut DocumentMut, version: &str) -> Result<()> {
    let requires_python = doc
        .get_mut("project")
        .and_then(|p| p.get_mut("requires-python"))
        .ok_or_else(|| {
            UvupError::CommandExecutionFailed(
                "No requires-python found in pyproject.toml".to_string(),
            )
        })?;

    *requires_python = Item::Value(Value::from(format!(">={version}")));

    Ok(())
}
