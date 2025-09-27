$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition

$projectPath = Join-Path $scriptDir "..\Diff"

$installDir = "C:\Tools\diff_tool"

if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir
}

Set-Location "$projectPath"

cargo build --release

$exeName = "diff_tool.exe"
Copy-Item "$projectPath\target\release\$exeName" -Destination $installDir -Force

# Adiciona ao PATH do usuário
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "User")
    Write-Host "Added $installDir to PATH. Restart terminal to use diff globally."
} else {
    Write-Host "$installDir is already in PATH."
}

Write-Host "Installation complete."
