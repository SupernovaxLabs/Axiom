$ErrorActionPreference = 'Stop'

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
$BinRoot = Join-Path $env:LOCALAPPDATA 'Axiom\\bin'

Push-Location $Root
try {
    Write-Host '[axiom] building release binaries...'
    cargo build --release -p axiom-cli -p axiom-interpreter

    New-Item -ItemType Directory -Path $BinRoot -Force | Out-Null
    Copy-Item (Join-Path $Root 'target\\release\\axiom-cli.exe') (Join-Path $BinRoot 'axiom.exe') -Force
    Copy-Item (Join-Path $Root 'target\\release\\axiom-interpreter.exe') (Join-Path $BinRoot 'axiom-interpreter.exe') -Force

    $currentUserPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    if (-not $currentUserPath) { $currentUserPath = '' }

    $parts = $currentUserPath.Split(';', [System.StringSplitOptions]::RemoveEmptyEntries)
    if ($parts -notcontains $BinRoot) {
        $newPath = if ($currentUserPath) { "$currentUserPath;$BinRoot" } else { $BinRoot }
        [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
        Write-Host "[axiom] added to user PATH: $BinRoot"
    }

    Write-Host "[axiom] installed binaries in: $BinRoot"
    Write-Host '[axiom] open a new terminal and run: axiom version'
}
finally {
    Pop-Location
}
