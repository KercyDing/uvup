// Allow println! in this module as it's used for user-facing output
#![allow(clippy::print_stdout)]

use crate::env::paths::get_env_path;
use crate::error::{Result, UvupError};
use crate::utils::print_success;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use toml_edit::{DocumentMut, Item, Value};

#[allow(clippy::too_many_arguments)]
pub(crate) fn run(
    template: String,
    python: Option<&str>,
    exclude: Option<&[String]>,
    include: Option<&[String]>,
    dry_run: bool,
) -> Result<()> {
    // Get current directory
    let current_dir = env::current_dir()
        .map_err(|e| UvupError::PathError(format!("Failed to get current directory: {e}")))?;

    // Check pyproject.toml exists in current directory
    let current_toml_path = current_dir.join("pyproject.toml");
    if !current_toml_path.exists() {
        return Err(UvupError::PathError(
            "No pyproject.toml found in current directory".to_string(),
        ));
    }

    // Check template exists
    let template_path = get_env_path(&template)?;
    if !template_path.exists() {
        return Err(UvupError::EnvNotFound(template));
    }

    // Read current and template pyproject.toml
    let current_doc = read_and_parse_toml(&current_toml_path)?;
    let template_doc = read_and_parse_toml(&template_path.join("pyproject.toml"))?;

    // Process template
    let mut synced_doc = current_doc.clone();

    // Sync dependencies from template
    sync_dependencies(&mut synced_doc, &template_doc, exclude, include);

    // Get Python versions
    let current_python = get_python_version_from_toml(&current_doc)?;
    let template_python = get_python_version_from_toml(&template_doc)?;
    let synced_python = if let Some(version) = python {
        update_python_version(&mut synced_doc, version)?;
        version.to_string()
    } else {
        current_python.clone()
    };

    // Dry-run mode
    if dry_run {
        print_dry_run_preview(
            &template,
            &current_dir,
            &current_doc,
            &synced_doc,
            &current_python,
            &template_python,
            &synced_python,
            exclude,
            include,
        );
        return Ok(());
    }

    // Sync project
    println!("Syncing project with template '{template}'...");

    // Backup current pyproject.toml
    let backup_path = current_dir.join("pyproject.toml.backup");
    fs::copy(&current_toml_path, &backup_path)
        .map_err(|e| UvupError::PathError(format!("Failed to backup pyproject.toml: {e}")))?;

    // Write updated pyproject.toml
    fs::write(&current_toml_path, synced_doc.to_string())
        .map_err(|e| UvupError::PathError(format!("Failed to write pyproject.toml: {e}")))?;

    // Lock and sync
    println!("Installing packages...");
    if let Err(e) = sync_environment(&current_dir) {
        // Restore backup on error
        let _ = fs::copy(&backup_path, &current_toml_path);
        let _ = fs::remove_file(&backup_path);
        return Err(e);
    }

    // Remove backup on success
    let _ = fs::remove_file(&backup_path);

    print_success(&format!(
        "Successfully synced project with template '{template}'"
    ));

    Ok(())
}

/// Read and parse pyproject.toml
fn read_and_parse_toml(path: &Path) -> Result<DocumentMut> {
    let toml_content = fs::read_to_string(path)
        .map_err(|e| UvupError::PathError(format!("Failed to read pyproject.toml: {e}")))?;

    toml_content.parse::<DocumentMut>().map_err(|e| {
        UvupError::CommandExecutionFailed(format!("Failed to parse pyproject.toml: {e}"))
    })
}

/// Lock and sync packages
fn sync_environment(project_path: &Path) -> Result<()> {
    println!("  Resolving and locking dependencies...");
    let lock_status = Command::new("uv")
        .arg("lock")
        .current_dir(project_path)
        .status()
        .map_err(|e| {
            UvupError::CommandExecutionFailed(format!("Failed to execute uv lock: {e}"))
        })?;

    if !lock_status.success() {
        return Err(UvupError::CommandExecutionFailed(
            "Failed to resolve and lock dependencies (possible version conflicts)".to_string(),
        ));
    }

    println!("  Installing locked packages...");
    let sync_status = Command::new("uv")
        .arg("sync")
        .current_dir(project_path)
        .status()
        .map_err(|e| {
            UvupError::CommandExecutionFailed(format!("Failed to execute uv sync: {e}"))
        })?;

    if !sync_status.success() {
        return Err(UvupError::CommandExecutionFailed(
            "Failed to install locked packages (possible network or permission issues)".to_string(),
        ));
    }

    Ok(())
}

/// Sync dependencies from template to current project
fn sync_dependencies(
    target_doc: &mut DocumentMut,
    template_doc: &DocumentMut,
    exclude: Option<&[String]>,
    include: Option<&[String]>,
) {
    // Get template dependencies
    if let Some(template_deps) = template_doc
        .get("project")
        .and_then(|p| p.get("dependencies"))
        .and_then(|d| d.as_array())
    {
        let filtered = filter_dependency_array(template_deps, exclude, include);

        // Update target dependencies
        if let Some(target_project) = target_doc.get_mut("project") {
            target_project["dependencies"] =
                Item::Value(Value::Array(toml_edit::Array::from_iter(filtered)));
        }
    }

    // Sync optional-dependencies
    if let Some(template_optional) = template_doc
        .get("project")
        .and_then(|p| p.get("optional-dependencies"))
        .and_then(|o| o.as_table())
    {
        let mut synced_optional = toml_edit::Table::new();

        for (group_name, group_deps) in template_optional {
            if let Some(deps_array) = group_deps.as_array() {
                let filtered = filter_dependency_array(deps_array, exclude, include);

                if filtered.is_empty() {
                    println!(
                        "  Note: Optional group '{group_name}' is empty after filtering, skipping"
                    );
                } else {
                    synced_optional.insert(
                        group_name,
                        Item::Value(Value::Array(toml_edit::Array::from_iter(filtered))),
                    );
                }
            }
        }

        // Update target optional-dependencies
        if let Some(target_project) = target_doc.get_mut("project") {
            if synced_optional.is_empty() {
                target_project
                    .as_table_mut()
                    .map(|t| t.remove("optional-dependencies"));
            } else {
                target_project["optional-dependencies"] = Item::Table(synced_optional);
            }
        }
    }
}

/// Filter a single dependency array
fn filter_dependency_array(
    deps_array: &toml_edit::Array,
    exclude: Option<&[String]>,
    include: Option<&[String]>,
) -> Vec<toml_edit::Value> {
    let mut filtered_deps = Vec::new();

    for dep in deps_array {
        if let Some(dep_str) = dep.as_str() {
            let package_name = extract_package_name(dep_str);

            if let Some(include_list) = include {
                let included = include_list
                    .iter()
                    .any(|inc| package_name == inc.to_lowercase());
                if !included {
                    continue;
                }
            }

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

/// Extract package name from dependency string
fn extract_package_name(dep_str: &str) -> String {
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

/// Print dry-run preview
#[allow(clippy::too_many_arguments)]
fn print_dry_run_preview(
    template: &str,
    current_dir: &Path,
    current_doc: &DocumentMut,
    synced_doc: &DocumentMut,
    current_python: &str,
    template_python: &str,
    synced_python: &str,
    exclude: Option<&[String]>,
    include: Option<&[String]>,
) {
    println!("-- Dry Run Mode --");
    println!();
    println!("Template: '{template}' (Python {template_python})");
    println!(
        "Current:  {} (Python {current_python})",
        current_dir.display()
    );
    if current_python != synced_python {
        println!("Synced:   Python {synced_python}");
    }
    println!();

    if current_python != synced_python {
        println!("Python version change:");
        println!("  {current_python} → {synced_python}");
        println!();
    }

    if exclude.is_some() || include.is_some() {
        println!("Filters applied:");
        if let Some(exc) = exclude {
            println!("  Exclude: {}", exc.join(", "));
        }
        if let Some(inc) = include {
            println!("  Include: {}", inc.join(", "));
        }
        println!();
    }

    println!("Dependency changes:");
    compare_dependencies(current_doc, synced_doc);
    println!();

    compare_optional_dependencies(current_doc, synced_doc);

    println!("To sync this project, run the same command without --dry-run");
}

/// Compare dependencies
fn compare_dependencies(current_doc: &DocumentMut, synced_doc: &DocumentMut) {
    let current_deps = extract_dependencies(current_doc);
    let synced_deps = extract_dependencies(synced_doc);

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut kept = Vec::new();

    for dep in &synced_deps {
        if current_deps.contains(dep) {
            kept.push(dep);
        } else {
            added.push(dep);
        }
    }

    for dep in &current_deps {
        if !synced_deps.contains(dep) {
            removed.push(dep);
        }
    }

    if added.is_empty() && removed.is_empty() {
        println!("  No changes to main dependencies");
    } else {
        if !added.is_empty() {
            println!("  Added ({}):", added.len());
            for dep in &added {
                println!("    + {dep}");
            }
        }
        if !removed.is_empty() {
            println!("  Removed ({}):", removed.len());
            for dep in &removed {
                println!("    - {dep}");
            }
        }
        if !kept.is_empty() {
            println!("  Kept ({}):", kept.len());
        }
    }
}

/// Compare optional-dependencies
fn compare_optional_dependencies(current_doc: &DocumentMut, synced_doc: &DocumentMut) {
    let current_optional = extract_optional_dependencies(current_doc);
    let synced_optional = extract_optional_dependencies(synced_doc);

    if current_optional.is_empty() && synced_optional.is_empty() {
        return;
    }

    println!("Optional dependencies:");

    let mut all_groups: std::collections::HashSet<String> =
        current_optional.keys().cloned().collect();
    all_groups.extend(synced_optional.keys().cloned());

    let mut groups: Vec<_> = all_groups.into_iter().collect();
    groups.sort();

    for group in groups {
        let current_deps = current_optional.get(&group);
        let synced_deps = synced_optional.get(&group);

        match (current_deps, synced_deps) {
            (Some(cur), Some(syn)) if cur == syn => {
                println!("  [{group}]: No changes");
            }
            (Some(_), Some(syn)) => {
                println!("  [{group}]: Modified ({} packages)", syn.len());
            }
            (Some(_), None) => {
                println!("  [{group}]: Removed");
            }
            (None, Some(syn)) => {
                println!("  [{group}]: Added ({} packages)", syn.len());
            }
            (None, None) => unreachable!(),
        }
    }
    println!();
}

/// Extract dependencies
fn extract_dependencies(doc: &DocumentMut) -> Vec<String> {
    doc.get("project")
        .and_then(|p| p.get("dependencies"))
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

/// Extract optional-dependencies
fn extract_optional_dependencies(
    doc: &DocumentMut,
) -> std::collections::HashMap<String, Vec<String>> {
    let mut result = std::collections::HashMap::new();

    if let Some(project) = doc.get("project") {
        if let Some(optional) = project.get("optional-dependencies") {
            if let Some(table) = optional.as_table() {
                for (key, value) in table {
                    if let Some(arr) = value.as_array() {
                        let deps: Vec<String> = arr
                            .iter()
                            .filter_map(|v| v.as_str())
                            .map(String::from)
                            .collect();
                        result.insert(key.to_string(), deps);
                    }
                }
            }
        }
    }

    result
}
