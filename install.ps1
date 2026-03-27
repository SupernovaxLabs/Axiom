param(
    [string]$Repo = 'axiom-lang/axiom',
    [string]$Version = 'latest',
    [string]$InstallRoot = "$env:LOCALAPPDATA\\Axiom",
    [switch]$FromSource
)

$ErrorActionPreference = 'Stop'
$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path

function Get-ReleaseMetadata {
    param(
        [string]$Repo,
        [string]$Version
    )

    $apiUrl = if ($Version -eq 'latest') {
        "https://api.github.com/repos/$Repo/releases/latest"
    }
    else {
        "https://api.github.com/repos/$Repo/releases/tags/$Version"
    }

    Write-Host "[axiom] querying $apiUrl"
    Invoke-RestMethod -Uri $apiUrl -Headers @{ 'User-Agent' = 'axiom-installer' }
}

function Select-WindowsAsset {
    param([object[]]$Assets)

    $patterns = @(
        'windows-x86_64',
        'windows-amd64',
        'win64',
        'windows'
    )

    foreach ($pattern in $patterns) {
        $asset = $Assets | Where-Object { $_.name -match $pattern -and $_.name -match '\\.zip$' } | Select-Object -First 1
        if ($asset) { return $asset }
    }

    $Assets | Where-Object { $_.name -match '\\.zip$' } | Select-Object -First 1
}

function Add-ToUserPath {
    param([string]$BinRoot)

    $currentUserPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    if (-not $currentUserPath) { $currentUserPath = '' }

    $parts = $currentUserPath.Split(';', [System.StringSplitOptions]::RemoveEmptyEntries)
    if ($parts -notcontains $BinRoot) {
        $newPath = if ($currentUserPath) { "$currentUserPath;$BinRoot" } else { $BinRoot }
        [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
        Write-Host "[axiom] added to user PATH: $BinRoot"
    }
}

function Install-FromArchive {
    param(
        [object]$Asset,
        [string]$InstallRoot
    )

    $tmpDir = Join-Path ([System.IO.Path]::GetTempPath()) ("axiom-install-" + [Guid]::NewGuid().ToString('N'))
    $archivePath = Join-Path $tmpDir $Asset.name
    $extractDir = Join-Path $tmpDir 'extract'
    $binRoot = Join-Path $InstallRoot 'bin'

    New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null
    New-Item -ItemType Directory -Path $extractDir -Force | Out-Null
    New-Item -ItemType Directory -Path $binRoot -Force | Out-Null

    try {
        Write-Host "[axiom] downloading $($Asset.browser_download_url)"
        Invoke-WebRequest -Uri $Asset.browser_download_url -OutFile $archivePath -Headers @{ 'User-Agent' = 'axiom-installer' }

        Write-Host "[axiom] extracting $($Asset.name)"
        Expand-Archive -Path $archivePath -DestinationPath $extractDir -Force

        $cli = Get-ChildItem -Path $extractDir -Recurse -Filter 'axiom-cli.exe' | Select-Object -First 1
        $interp = Get-ChildItem -Path $extractDir -Recurse -Filter 'axiom-interpreter.exe' | Select-Object -First 1

        if (-not $cli -or -not $interp) {
            throw "Could not find axiom-cli.exe and axiom-interpreter.exe in release asset '$($Asset.name)'"
        }

        Copy-Item $cli.FullName (Join-Path $binRoot 'axiom.exe') -Force
        Copy-Item $interp.FullName (Join-Path $binRoot 'axiom-interpreter.exe') -Force

        Add-ToUserPath -BinRoot $binRoot

        Write-Host "[axiom] installed binaries in: $binRoot"
        Write-Host "[axiom] open a new terminal and run: axiom version"
    }
    finally {
        if (Test-Path $tmpDir) {
            Remove-Item -Recurse -Force $tmpDir
        }
    }
}

function Install-FromSource {
    param([string]$InstallRoot)

    $repoRoot = $ScriptRoot
    $binRoot = Join-Path $InstallRoot 'bin'

    Push-Location $repoRoot
    try {
        Write-Host '[axiom] building release binaries from source...'
        cargo build --release -p axiom-cli -p axiom-interpreter

        New-Item -ItemType Directory -Path $binRoot -Force | Out-Null
        Copy-Item (Join-Path $repoRoot 'target\\release\\axiom-cli.exe') (Join-Path $binRoot 'axiom.exe') -Force
        Copy-Item (Join-Path $repoRoot 'target\\release\\axiom-interpreter.exe') (Join-Path $binRoot 'axiom-interpreter.exe') -Force

        Add-ToUserPath -BinRoot $binRoot
        Write-Host "[axiom] installed binaries in: $binRoot"
        Write-Host '[axiom] open a new terminal and run: axiom version'
    }
    finally {
        Pop-Location
    }
}

try {
    if ($FromSource) {
        Install-FromSource -InstallRoot $InstallRoot
        exit 0
    }

    $release = Get-ReleaseMetadata -Repo $Repo -Version $Version
    $asset = Select-WindowsAsset -Assets $release.assets

    if (-not $asset) {
        throw "No downloadable Windows zip asset found for release '$($release.tag_name)'. Re-run with -FromSource."
    }

    Write-Host "[axiom] installing from release $($release.tag_name): $($asset.name)"
    Install-FromArchive -Asset $asset -InstallRoot $InstallRoot
}
catch {
    Write-Warning "[axiom] release install failed: $($_.Exception.Message)"
    Write-Host '[axiom] falling back to source build install...'
    Install-FromSource -InstallRoot $InstallRoot
}
