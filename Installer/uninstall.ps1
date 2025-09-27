$installDir = "C:\Tools\diff_tool"

if (Test-Path $installDir) {
    Remove-Item -Path $installDir -Recurse -Force
    Write-Host "Removed installation folder: $installDir"
} else {
    Write-Host "Installation folder not found: $installDir"
}

$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
$paths = $currentPath -split ';' | Where-Object { $_ -ne $installDir }
$newPath = ($paths -join ';')
[Environment]::SetEnvironmentVariable("Path", $newPath, "User")
Write-Host "Removed $installDir from user PATH."

Write-Host "Uninstallation complete. Please restart your terminal."
