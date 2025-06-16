$ErrorActionPreference = "Stop"

$Version = $args[0] -replace "^v", ""
if (!$Version) {
    throw "usage: $PSCommandPath <version> [<target>]"
}
$Target = $args[1]
if (!$Target) {
    $Target = $(rustc.exe -vV) -match "^host: (.*)" -replace "^host: ", ""
}
$BuildProfile = "release-dist"

$BaseDirectory = Split-Path $PSScriptRoot -Parent
$ArchiveDirectory = Join-Path $PSScriptRoot "GameController-$Version-$Target"
$Archive = Join-Path $PSScriptRoot "GameController-$Version-$Target.zip"

if (Test-Path $ArchiveDirectory) {
    Remove-Item -Recurse -Force $ArchiveDirectory
}

Push-Location $(Join-Path $BaseDirectory "frontend")
npm ci
npm run build
Pop-Location

Push-Location $BaseDirectory
cargo build --target $Target --profile $BuildProfile --package game_controller_app
Pop-Location

New-Item -ItemType Directory -Path $(Join-Path $ArchiveDirectory "target\release")
Copy-Item $(Join-Path $BaseDirectory "LICENSE") $ArchiveDirectory
Copy-Item $(Join-Path $BaseDirectory "README.md") $ArchiveDirectory
Copy-Item $(Join-Path $BaseDirectory "config") $ArchiveDirectory -Recurse
Copy-Item $(Join-Path $BaseDirectory "target\$Target\$BuildProfile\game_controller_app.exe") $(Join-Path $ArchiveDirectory "target\release")
New-Item -ItemType File -Path $(Join-Path $ArchiveDirectory "GameController.bat") -Value @"
@echo off
start %~dp0\target\release\game_controller_app.exe %*
"@
Compress-Archive $ArchiveDirectory $Archive -Force
