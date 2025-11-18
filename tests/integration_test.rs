use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_test_env_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".uvup")
}

fn cleanup_test_env(name: &str) {
    let env_path = get_test_env_dir().join(name);
    if env_path.exists() {
        fs::remove_dir_all(env_path).ok();
    }
}

#[test]
fn test_init_command() {
    // Test --raw flag to get shell script output
    let output = Command::new("cargo")
        .args(["run", "--", "init", "--raw"])
        .output()
        .expect("Failed to execute uvup init --raw");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check for platform-specific shell hook content
    #[cfg(target_os = "windows")]
    {
        assert!(stdout.contains("function uvup"));
    }

    #[cfg(not(target_os = "windows"))]
    {
        assert!(stdout.contains("uvup()"));
    }

    // Common assertions for all platforms
    assert!(stdout.contains("activate"));
    assert!(stdout.contains("deactivate"));
}

#[test]
fn test_create_list_delete_workflow() {
    let test_env = "test-integration-env";

    cleanup_test_env(test_env);

    let create_output = Command::new("cargo")
        .args(["run", "--", "create", test_env])
        .output()
        .expect("Failed to execute uvup create");

    assert!(create_output.status.success());
    let create_stdout = String::from_utf8_lossy(&create_output.stdout);
    assert!(create_stdout.contains(&format!("Environment '{test_env}' created successfully")));

    let env_path = get_test_env_dir().join(test_env);
    assert!(env_path.exists());

    let list_output = Command::new("cargo")
        .args(["run", "--", "list"])
        .output()
        .expect("Failed to execute uvup list");

    assert!(list_output.status.success());
    let list_stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(list_stdout.contains(test_env));

    let delete_output = Command::new("cargo")
        .args(["run", "--", "delete", test_env])
        .output()
        .expect("Failed to execute uvup delete");

    assert!(delete_output.status.success());
    let delete_stdout = String::from_utf8_lossy(&delete_output.stdout);
    assert!(delete_stdout.contains(&format!("Environment '{test_env}' removed")));

    assert!(!env_path.exists());
}

#[test]
fn test_create_with_python_version() {
    let test_env = "test-python-version";

    cleanup_test_env(test_env);

    let create_output = Command::new("cargo")
        .args(["run", "--", "create", test_env, "--python", "3.11"])
        .output()
        .expect("Failed to execute uvup create with python version");

    assert!(create_output.status.success());

    let env_path = get_test_env_dir().join(test_env);
    assert!(env_path.exists());

    cleanup_test_env(test_env);
}

#[test]
fn test_create_duplicate_environment() {
    let test_env = "test-duplicate-env";

    cleanup_test_env(test_env);

    Command::new("cargo")
        .args(["run", "--", "create", test_env])
        .output()
        .expect("Failed to execute first uvup create");

    let duplicate_output = Command::new("cargo")
        .args(["run", "--", "create", test_env])
        .output()
        .expect("Failed to execute second uvup create");

    assert!(!duplicate_output.status.success());
    let stderr = String::from_utf8_lossy(&duplicate_output.stderr);
    assert!(stderr.contains("already exists"));

    cleanup_test_env(test_env);
}

#[test]
fn test_delete_nonexistent_environment() {
    let test_env = "nonexistent-env";

    cleanup_test_env(test_env);

    let delete_output = Command::new("cargo")
        .args(["run", "--", "delete", test_env])
        .output()
        .expect("Failed to execute uvup delete");

    assert!(!delete_output.status.success());
    let stderr = String::from_utf8_lossy(&delete_output.stderr);
    assert!(stderr.contains("not found"));
}

#[test]
fn test_invalid_environment_name() {
    let invalid_names = vec!["my env", "my/env", "my.env", ""];

    for invalid_name in invalid_names {
        if invalid_name.is_empty() {
            continue;
        }

        let create_output = Command::new("cargo")
            .args(["run", "--", "create", invalid_name])
            .output()
            .expect("Failed to execute uvup create with invalid name");

        assert!(!create_output.status.success());
        let stderr = String::from_utf8_lossy(&create_output.stderr);
        assert!(stderr.contains("Invalid environment name"));

        cleanup_test_env(invalid_name);
    }
}

#[test]
fn test_list_empty_environments() {
    let test_env = "test-list-cleanup";
    cleanup_test_env(test_env);

    let envs_dir = get_test_env_dir();
    if envs_dir.exists() {
        let entries = fs::read_dir(&envs_dir).unwrap();
        if entries.count() == 0 {
            let list_output = Command::new("cargo")
                .args(["run", "--", "list"])
                .output()
                .expect("Failed to execute uvup list");

            assert!(list_output.status.success());
            let stdout = String::from_utf8_lossy(&list_output.stdout);
            assert!(stdout.contains("No environments found") || stdout.is_empty());
        }
    }
}

// Package management tests
// Note: These tests require UVUP_ACTIVE_ENV to be set, which requires shell integration.
// They are marked as ignored by default and can be run with: cargo test -- --ignored

#[test]
#[ignore = "requires shell integration to set UVUP_ACTIVE_ENV"]
fn test_add_package_without_activation() {
    let add_output = Command::new("cargo")
        .args(["run", "--", "add", "requests"])
        .output()
        .expect("Failed to execute uvup add");

    assert!(!add_output.status.success());
    let stderr = String::from_utf8_lossy(&add_output.stderr);
    assert!(stderr.contains("No active environment"));
}

#[test]
#[ignore = "requires shell integration to set UVUP_ACTIVE_ENV"]
fn test_remove_package_without_activation() {
    let remove_output = Command::new("cargo")
        .args(["run", "--", "remove", "requests"])
        .output()
        .expect("Failed to execute uvup remove");

    assert!(!remove_output.status.success());
    let stderr = String::from_utf8_lossy(&remove_output.stderr);
    assert!(stderr.contains("No active environment"));
}

#[test]
#[ignore = "requires shell integration to set UVUP_ACTIVE_ENV"]
fn test_lock_without_activation() {
    let lock_output = Command::new("cargo")
        .args(["run", "--", "lock"])
        .output()
        .expect("Failed to execute uvup lock");

    assert!(!lock_output.status.success());
    let stderr = String::from_utf8_lossy(&lock_output.stderr);
    assert!(stderr.contains("No active environment"));
}

#[test]
#[ignore = "requires shell integration to set UVUP_ACTIVE_ENV"]
fn test_tree_without_activation() {
    let tree_output = Command::new("cargo")
        .args(["run", "--", "tree"])
        .output()
        .expect("Failed to execute uvup tree");

    assert!(!tree_output.status.success());
    let stderr = String::from_utf8_lossy(&tree_output.stderr);
    assert!(stderr.contains("No active environment"));
}

#[test]
fn test_clone_environment() {
    let source_env = "test-clone-source";
    let target_env = "test-clone-target";

    cleanup_test_env(source_env);
    cleanup_test_env(target_env);

    // Create source environment
    Command::new("cargo")
        .args(["run", "--", "create", source_env])
        .output()
        .expect("Failed to create source environment");

    // Clone it
    let clone_output = Command::new("cargo")
        .args(["run", "--", "clone", source_env, target_env])
        .output()
        .expect("Failed to execute uvup clone");

    assert!(clone_output.status.success());

    // Verify target exists
    let target_path = get_test_env_dir().join(target_env);
    assert!(target_path.exists());
    assert!(target_path.join("pyproject.toml").exists());
    assert!(target_path.join(".venv").exists());

    // Cleanup
    cleanup_test_env(source_env);
    cleanup_test_env(target_env);
}

#[test]
fn test_clone_nonexistent_environment() {
    let source_env = "nonexistent-source";
    let target_env = "test-clone-target-2";

    cleanup_test_env(source_env);
    cleanup_test_env(target_env);

    let clone_output = Command::new("cargo")
        .args(["run", "--", "clone", source_env, target_env])
        .output()
        .expect("Failed to execute uvup clone");

    assert!(!clone_output.status.success());
    let stderr = String::from_utf8_lossy(&clone_output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("does not exist"));

    cleanup_test_env(target_env);
}
