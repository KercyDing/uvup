pub(crate) const FISH_HOOK: &str = r#"
function uvup
    if test "$argv[1]" = "activate"
        if test -z "$argv[2]"
            echo "Error: Please specify environment name"
            echo "Usage: uvup activate <name>"
            return 1
        end

        set -l env_path "$HOME/.uvup/$argv[2]"
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

    else if test "$argv[1]" = "deactivate"
        if functions -q deactivate
            deactivate
        else
            echo "Error: No active environment"
            return 1
        end
    else
        command uvup $argv
    end
end
"#;
