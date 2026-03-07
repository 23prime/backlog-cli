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
        & $BlExe auth logout 2>$null
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
