pub(crate) const POWERSHELL_HOOK: &str = r#"
function uvup {
    param([string[]]$Arguments)

    if ($Arguments[0] -eq "activate") {
        if (-not $Arguments[1]) {
            Write-Host "Error: Please specify environment name" -ForegroundColor Red
            Write-Host "Usage: uvup activate <name>"
            return
        }

        $envPath = "$env:USERPROFILE\.uvup\$($Arguments[1])"
        $activateScript = "$envPath\Scripts\Activate.ps1"

        if (-not (Test-Path $activateScript)) {
            Write-Host "Error: Environment '$($Arguments[1])' not found" -ForegroundColor Red
            Write-Host "Tip: Use 'uvup list' to see all available environments"
            return
        }

        if ($env:VIRTUAL_ENV) {
            deactivate
        }

        & $activateScript

    } elseif ($Arguments[0] -eq "deactivate") {
        if (Get-Command deactivate -ErrorAction SilentlyContinue) {
            deactivate
        } else {
            Write-Host "Error: No active environment" -ForegroundColor Red
        }
    } else {
        & uvup.exe $Arguments
    }
}
"#;
