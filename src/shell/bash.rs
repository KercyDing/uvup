pub(crate) const BASH_HOOK: &str = r#"
uvup() {
    if [ "$1" = "activate" ]; then
        if [ -z "$2" ]; then
            echo "Error: Please specify environment name"
            echo "Usage: uvup activate <name>"
            return 1
        fi

        local env_path="$HOME/.uvup/$2/.venv"
        local activate_script=""

        # Check for Windows (Git Bash) or Unix paths
        if [ -f "$env_path/Scripts/activate" ]; then
            activate_script="$env_path/Scripts/activate"
        elif [ -f "$env_path/bin/activate" ]; then
            activate_script="$env_path/bin/activate"
        fi

        if [ -z "$activate_script" ]; then
            echo "Error: Environment '$2' not found"
            echo "Tip: Use 'uvup list' to see all available environments"
            return 1
        fi

        if [ -n "$VIRTUAL_ENV" ]; then
            if type deactivate > /dev/null 2>&1; then
                deactivate
            fi
        fi

        source "$activate_script"
        export UVUP_ACTIVE_ENV="$2"

    elif [ "$1" = "deactivate" ]; then
        if [ -n "$2" ]; then
            echo "Error: Unknown command 'uvup deactivate $2'"
            echo "Did you mean: uvup deactivate"
            return 1
        fi

        if type deactivate > /dev/null 2>&1; then
            deactivate
            unset UVUP_ACTIVE_ENV
        else
            echo "Error: No active environment"
            return 1
        fi
    else
        command uvup "$@"
    fi
}
"#;
