pub(crate) const FISH_HOOK: &str = r#"
function uvup
    if test "$argv[1]" = "activate"
        if test -z "$argv[2]"
            echo "Error: Please specify environment name"
            echo "Usage: uvup activate <name>"
            return 1
        end

        set -l root "$UVUP_HOME"
        if test -z "$root"
            set root "$HOME/.uvup"
        end
        set -l env_path "$root/$argv[2]/.venv"
        set -l activate_script "$env_path/bin/activate.fish"

        if not test -f "$activate_script"
            echo "Error: Environment '$argv[2]' not found"
            echo "Tip: Use 'uvup list' to see all available environments"
            return 1
        end

        if set -q VIRTUAL_ENV
            deactivate
        end

        source "$activate_script"
        set -gx UVUP_ACTIVE_ENV "$argv[2]"

    else if test "$argv[1]" = "deactivate"
        if test -n "$argv[2]"
            echo "Error: Unknown command 'uvup deactivate $argv[2]'"
            echo "Did you mean: uvup deactivate"
            return 1
        end

        if functions -q deactivate
            deactivate
            set -e UVUP_ACTIVE_ENV
        else
            echo "Error: No active environment"
            return 1
        end
    else
        command uvup $argv
    end
end
"#;
