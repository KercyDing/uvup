pub(crate) const POWERSHELL_HOOK: &str = r#"
function uvup {
    if ($args.Count -eq 0) {
        & uvup.exe
        return
    }

    if ($args[0] -eq "activate") {
        if (-not $args[1]) {
            Write-Host "Error: Please specify environment name" -ForegroundColor Red
            Write-Host "Usage: uvup activate <name>"
            return
        }

        $root = if ($env:UVUP_HOME) { $env:UVUP_HOME } else { "$env:USERPROFILE\.uvup" }
        $envPath = "$root\$($args[1])\.venv"
        $activateScript = "$envPath\Scripts\Activate.ps1"

        if (-not (Test-Path $activateScript)) {
            Write-Host "Error: Environment '$($args[1])' not found" -ForegroundColor Red
            Write-Host "Tip: Use 'uvup list' to see all available environments"
            return
        }

        if ($env:VIRTUAL_ENV) {
            deactivate
        }

        & $activateScript
        $env:UVUP_ACTIVE_ENV = $args[1]

    } elseif ($args[0] -eq "deactivate") {
        if ($args[1]) {
            Write-Host "Error: Unknown command 'uvup deactivate $($args[1])'" -ForegroundColor Red
            Write-Host "Did you mean: uvup deactivate"
            return
        }

        if (Get-Command deactivate -ErrorAction SilentlyContinue) {
            deactivate
            Remove-Item Env:\UVUP_ACTIVE_ENV -ErrorAction SilentlyContinue
        } else {
            Write-Host "Error: No active environment" -ForegroundColor Red
        }
    } else {
        & uvup.exe @args
    }
}
"#;
