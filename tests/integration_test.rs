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
    let output = Command::new("cargo")
        .args(["run", "--", "init"])
        .output()
        .expect("Failed to execute uvup init");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("uvup()"));
    assert!(stdout.contains("activate"));
    assert!(stdout.contains("deactivate"));
}

#[test]
fn test_create_list_remove_workflow() {
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

    let remove_output = Command::new("cargo")
        .args(["run", "--", "remove", test_env])
        .output()
        .expect("Failed to execute uvup remove");

    assert!(remove_output.status.success());
    let remove_stdout = String::from_utf8_lossy(&remove_output.stdout);
    assert!(remove_stdout.contains(&format!("Environment '{test_env}' removed")));

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
fn test_remove_nonexistent_environment() {
    let test_env = "nonexistent-env";

    cleanup_test_env(test_env);

    let remove_output = Command::new("cargo")
        .args(["run", "--", "remove", test_env])
        .output()
        .expect("Failed to execute uvup remove");

    assert!(!remove_output.status.success());
    let stderr = String::from_utf8_lossy(&remove_output.stderr);
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
