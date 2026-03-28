param(
    [string]$Repo = 'SupernovaxLabs/Axiom',
    [string]$Version = 'latest',
    [string]$InstallRoot = "$env:LOCALAPPDATA\Axiom"
)

$ErrorActionPreference = 'Stop'

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

function Install-FromRelease {
    param(
        [object]$Release,
        [string]$InstallRoot
    )

    $binRoot = Join-Path $InstallRoot 'bin'
    New-Item -ItemType Directory -Path $binRoot -Force | Out-Null

    $cliAsset = $Release.assets | Where-Object { $_.name -eq 'axiom-cli.exe' } | Select-Object -First 1
    $interpAsset = $Release.assets | Where-Object { $_.name -eq 'axiom-interpreter.exe' } | Select-Object -First 1

    if (-not $cliAsset -or -not $interpAsset) {
        throw "Release '$($Release.tag_name)' is missing required assets (axiom-cli.exe and/or axiom-interpreter.exe)."
    }

    $headers = @{ 'User-Agent' = 'axiom-installer' }

    Write-Host "[axiom] downloading $($cliAsset.browser_download_url)"
    Invoke-WebRequest -Uri $cliAsset.browser_download_url -OutFile (Join-Path $binRoot 'axiom.exe') -Headers $headers

    Write-Host "[axiom] downloading $($interpAsset.browser_download_url)"
    Invoke-WebRequest -Uri $interpAsset.browser_download_url -OutFile (Join-Path $binRoot 'axiom-interpreter.exe') -Headers $headers

    Add-ToUserPath -BinRoot $binRoot

    Write-Host "[axiom] installed binaries in: $binRoot"
    Write-Host "[axiom] open a new terminal and run: axiom version"
}

$release = Get-ReleaseMetadata -Repo $Repo -Version $Version
Write-Host "[axiom] installing from release $($release.tag_name)"
Install-FromRelease -Release $release -InstallRoot $InstallRoot
