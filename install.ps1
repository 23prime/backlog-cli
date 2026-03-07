#Requires -Version 5.1
[CmdletBinding()]
param(
    [string]$InstallDir = "$env:USERPROFILE\.local\bin"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$Repo = '23prime/backlog-cli'
$AssetName = 'bl-x86_64-pc-windows-msvc.zip'

# Get latest release tag
Write-Host 'Fetching latest release...'
try {
    $release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
} catch {
    Write-Error "Failed to fetch release info from GitHub API: $_"
}
$tag = $release.tag_name
if (-not $tag) {
    Write-Error 'Failed to fetch latest release tag.'
}
Write-Host "Latest version: $tag"

# Download binary and checksum
$baseUrl = "https://github.com/$Repo/releases/download/$tag"
$zipUrl  = "$baseUrl/$AssetName"
$sha256Url = "$zipUrl.sha256"

$tmpDir = Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Path $tmpDir | Out-Null

try {
    $zipPath    = Join-Path $tmpDir $AssetName
    $sha256Path = "$zipPath.sha256"

    Write-Host "Downloading $zipUrl..."
    Invoke-WebRequest $zipUrl    -OutFile $zipPath    -UseBasicParsing
    Invoke-WebRequest $sha256Url -OutFile $sha256Path -UseBasicParsing

    # Verify checksum
    Write-Host 'Verifying checksum...'
    $expected = (Get-Content $sha256Path -Raw).Trim().Split()[0].ToLower()
    $actual   = (Get-FileHash $zipPath -Algorithm SHA256).Hash.ToLower()
    if ($expected -ne $actual) {
        Write-Error "Checksum mismatch!`n  expected: $expected`n  actual:   $actual"
    }

    # Extract and install
    Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir | Out-Null
    }
    Copy-Item (Join-Path $tmpDir 'bl.exe') (Join-Path $InstallDir 'bl.exe') -Force

    Write-Host "Installed bl to $InstallDir\bl.exe"

    # PATH hint
    $effectivePathEntries = ($env:PATH -split ';') | Where-Object { $_ }
    if ($effectivePathEntries -notcontains $InstallDir) {
        $userPath = [System.Environment]::GetEnvironmentVariable('PATH', 'User')
        $newUserPath = if ([string]::IsNullOrWhiteSpace($userPath)) { $InstallDir } else { "$InstallDir;$userPath" }
        Write-Host ""
        Write-Host "Note: $InstallDir is not in your PATH. Add it with:"
        Write-Host "  [System.Environment]::SetEnvironmentVariable('PATH', `"$newUserPath`", 'User')"
    }
} finally {
    Remove-Item -Recurse -Force $tmpDir -ErrorAction SilentlyContinue
}
