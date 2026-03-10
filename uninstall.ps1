#Requires -Version 5.1
[CmdletBinding()]
param(
    [string]$InstallDir = "$env:USERPROFILE\.local\bin",
    [switch]$Purge
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$BlExe    = Join-Path $InstallDir 'bl.exe'
$ConfigDir = Join-Path $env:APPDATA 'bl'

if ($Purge) {
    if (Test-Path $BlExe) {
        Write-Host 'Removing credentials...'
        try { & $BlExe auth logout --all 2>$null } catch { }
    } else {
        Write-Host "Warning: bl.exe not found at $BlExe; credentials were not removed automatically."
        Write-Host "Run 'bl auth logout --all' from the original install location to clear credentials."
    }

    if (Test-Path $ConfigDir) {
        Remove-Item -Recurse -Force $ConfigDir
        Write-Host "Removed $ConfigDir"
    }
}

if (Test-Path $BlExe) {
    Remove-Item -Force $BlExe
    Write-Host "Removed $BlExe"
} else {
    Write-Host "bl.exe not found at $BlExe"
}

Write-Host 'Done.'
