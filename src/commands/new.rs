// Allow println! in this module as it's used for user-facing output
#![allow(clippy::print_stdout)]

use crate::env::paths::{get_env_path, validate_env_name};
use crate::error::{Result, UvupError};
use crate::utils::print_success;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml_edit::{DocumentMut, Item, Value};

#[allow(clippy::too_many_arguments)]
pub(crate) fn run(
    name: &str,
    template: String,
    python: Option<&str>,
    exclude: Option<&[String]>,
    include: Option<&[String]>,
    path: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    // Validate template name
    validate_env_name(&template)?;

    // Check template exists
    let template_path = get_env_path(&template)?;
    if !template_path.exists() {
        return Err(UvupError::EnvNotFound(template));
    }

    // Determine project path
    let project_path = if let Some(p) = path {
        PathBuf::from(p).join(name)
    } else {
        env::current_dir()
            .map_err(|e| UvupError::PathError(format!("Failed to get current directory: {e}")))?
            .join(name)
    };

    // Check project doesn't exist
    if project_path.exists() {
        return Err(UvupError::PathError(format!(
            "Directory '{}' already exists",
            project_path.display()
        )));
    }

    // Read and process template pyproject.toml
    let template_doc = read_and_parse_toml(&template_path)?;
    let mut project_doc = template_doc.clone();

    // Apply filters
    if exclude.is_some() || include.is_some() {
        filter_dependencies(&mut project_doc, exclude, include)?;
    }

    // Get Python versions
    let template_python = get_python_version_from_toml(&template_doc)?;
    let project_python = if let Some(version) = python {
        update_python_version(&mut project_doc, version)?;
        version.to_string()
    } else {
        template_python.clone()
    };

    // Update project name in pyproject.toml
    if let Some(project_table) = project_doc.get_mut("project") {
        if let Some(name_item) = project_table.get_mut("name") {
            *name_item = Item::Value(Value::from(name));
        }
    }

    // Dry-run mode
    if dry_run {
        print_dry_run_preview(
            &template,
            name,
            &project_path,
            &template_doc,
            &project_doc,
            &template_python,
            &project_python,
            exclude,
            include,
        );
        return Ok(());
    }

    // Create project
    println!("Creating project '{name}' from template '{template}'...");

    fs::create_dir_all(&project_path)?;

    // Write pyproject.toml
    fs::write(project_path.join("pyproject.toml"), project_doc.to_string())
        .map_err(|e| UvupError::PathError(format!("Failed to write pyproject.toml: {e}")))?;

    // Create venv
    println!("Creating virtual environment with Python {project_python}...");
    let venv_status = Command::new("uv")
        .arg("venv")
        .current_dir(&project_path)
        .status()
        .map_err(|e| {
            UvupError::CommandExecutionFailed(format!("Failed to execute uv venv: {e}"))
        })?;

    if !venv_status.success() {
        let _ = fs::remove_dir_all(&project_path);
        return Err(UvupError::CommandExecutionFailed(
            "Failed to create virtual environment".to_string(),
        ));
    }

    // Lock and sync
    println!("Installing packages...");
    sync_environment(&project_path)?;

    print_success(&format!(
        "Successfully created project '{name}' from template '{template}'"
    ));
    println!("Project location: {}", project_path.display());

    Ok(())
}

/// Read and parse pyproject.toml
fn read_and_parse_toml(path: &Path) -> Result<DocumentMut> {
    let toml_path = path.join("pyproject.toml");
    let toml_content = fs::read_to_string(&toml_path)
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
        let _ = fs::remove_dir_all(project_path);
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
        let _ = fs::remove_dir_all(project_path);
        return Err(UvupError::CommandExecutionFailed(
            "Failed to install locked packages (possible network or permission issues)".to_string(),
        ));
    }

    Ok(())
}

/// Filter dependencies
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

    // Filter optional-dependencies
    if let Some(project) = doc.get_mut("project") {
        if let Some(optional_deps) = project.get_mut("optional-dependencies") {
            if let Some(optional_table) = optional_deps.as_table_mut() {
                let mut empty_groups = Vec::new();

                for (group_name, group_deps) in optional_table.iter_mut() {
                    if let Some(deps_array) = group_deps.as_array_mut() {
                        let filtered = filter_dependency_array(deps_array, exclude, include);

                        if filtered.is_empty() {
                            println!(
                                "  Note: Optional group '{group_name}' is now empty after filtering"
                            );
                            empty_groups.push(group_name.to_string());
                        } else {
                            *deps_array = toml_edit::Array::from_iter(filtered);
                        }
                    }
                }

                for group in empty_groups {
                    optional_table.remove(&group);
                }
            }
        }
    }

    Ok(())
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
    name: &str,
    project_path: &Path,
    template_doc: &DocumentMut,
    project_doc: &DocumentMut,
    template_python: &str,
    project_python: &str,
    exclude: Option<&[String]>,
    include: Option<&[String]>,
) {
    println!("-- Dry Run Mode --");
    println!();
    println!("Template: '{template}' (Python {template_python})");
    println!("Project:  '{name}' (Python {project_python})");
    println!("Location: {}", project_path.display());
    println!();

    if template_python != project_python {
        println!("Python version change:");
        println!("  {template_python} → {project_python}");
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
    compare_dependencies(template_doc, project_doc);
    println!();

    compare_optional_dependencies(template_doc, project_doc);

    println!("To create this project, run the same command without --dry-run");
}

/// Compare dependencies
fn compare_dependencies(template_doc: &DocumentMut, project_doc: &DocumentMut) {
    let template_deps = extract_dependencies(template_doc);
    let project_deps = extract_dependencies(project_doc);

    let mut removed = Vec::new();
    let mut kept = Vec::new();

    for dep in &template_deps {
        if project_deps.contains(dep) {
            kept.push(dep);
        } else {
            removed.push(dep);
        }
    }

    if removed.is_empty() {
        println!("  No changes to main dependencies");
    } else {
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
fn compare_optional_dependencies(template_doc: &DocumentMut, project_doc: &DocumentMut) {
    let template_optional = extract_optional_dependencies(template_doc);
    let project_optional = extract_optional_dependencies(project_doc);

    if template_optional.is_empty() && project_optional.is_empty() {
        return;
    }

    println!("Optional dependencies:");

    let mut all_groups: std::collections::HashSet<String> =
        template_optional.keys().cloned().collect();
    all_groups.extend(project_optional.keys().cloned());

    let mut groups: Vec<_> = all_groups.into_iter().collect();
    groups.sort();

    for group in groups {
        let template_deps = template_optional.get(&group);
        let project_deps = project_optional.get(&group);

        match (template_deps, project_deps) {
            (Some(src), Some(tgt)) if src == tgt => {
                println!("  [{group}]: No changes");
            }
            (Some(_), Some(tgt)) => {
                println!("  [{group}]: Modified ({} packages)", tgt.len());
            }
            (Some(_), None) => {
                println!("  [{group}]: Removed (group is empty after filtering)");
            }
            (None, Some(tgt)) => {
                println!("  [{group}]: Added ({} packages)", tgt.len());
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
