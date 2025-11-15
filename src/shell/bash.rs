pub(crate) const BASH_HOOK: &str = r#"
uvup() {
    if [ "$1" = "activate" ]; then
        if [ -z "$2" ]; then
            echo "Error: Please specify environment name"
            echo "Usage: uvup activate <name>"
            return 1
        fi

        local env_path="$HOME/.uvup/$2"
        local activate_script="$env_path/bin/activate"

        if [ ! -f "$activate_script" ]; then
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

    elif [ "$1" = "deactivate" ]; then
        if type deactivate > /dev/null 2>&1; then
            deactivate
        else
            echo "Error: No active environment"
            return 1
        fi
    else
        command uvup "$@"
    fi
}
"#;
