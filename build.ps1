[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

try {
    $sourcePath = Join-Path $PSScriptRoot 'credits-ex'
    $manifestPath = Join-Path $sourcePath 'Cargo.toml'
    $audioPath = Join-Path $PSScriptRoot 'audio\credits-ex.wav'
    $outputPath = Join-Path $PSScriptRoot 'dist\credits-ex'

    foreach ($inputPath in @($manifestPath, $audioPath)) {
        if (-not (Test-Path -LiteralPath $inputPath -PathType Leaf)) {
            throw "Required file not found: $inputPath"
        }
    }

    $cargo = Get-Command cargo.exe -CommandType Application -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($null -eq $cargo) {
        throw 'Cargo was not found. Install Rust and make cargo.exe available on PATH.'
    }

    Write-Host 'Building credits-ex (release)...'
    $rustFlags = $env:RUSTFLAGS
    $encodedRustFlags = $env:CARGO_ENCODED_RUSTFLAGS
    Push-Location -LiteralPath $sourcePath

    try {
        $env:RUSTFLAGS = "$rustFlags -Awarnings".Trim()
        if ($null -ne $encodedRustFlags) {
            $env:CARGO_ENCODED_RUSTFLAGS = if ($encodedRustFlags.Length -gt 0) {
                $encodedRustFlags + [char]0x1f + '-Awarnings'
            }
            else {
                '-Awarnings'
            }
        }
        $messages = @(& $cargo.Source build --release --locked --manifest-path $manifestPath --message-format=json-render-diagnostics)
        if ($LASTEXITCODE -ne 0) {
            throw "Cargo build failed with exit code $LASTEXITCODE."
        }
    }
    
    finally {
        $env:RUSTFLAGS = $rustFlags
        $env:CARGO_ENCODED_RUSTFLAGS = $encodedRustFlags
        Pop-Location
    }

    $executablePath = $null
    foreach ($message in $messages) {
        $artifact = $message | ConvertFrom-Json
        if ($artifact.reason -eq 'compiler-artifact' -and $artifact.target.name -eq 'credits-ex' -and $artifact.target.kind -contains 'bin' -and $artifact.executable) {
            $executablePath = $artifact.executable
        }
    }
    if ($null -eq $executablePath -or -not (Test-Path -LiteralPath $executablePath -PathType Leaf)) {
        throw 'Cargo did not produce the credits-ex executable.'
    }

    $outputAudioPath = Join-Path $outputPath 'audio'
    New-Item -ItemType Directory -Path $outputAudioPath -Force | Out-Null
    Copy-Item -LiteralPath $executablePath -Destination (Join-Path $outputPath 'credits-ex.exe') -Force
    Copy-Item -LiteralPath $audioPath -Destination (Join-Path $outputAudioPath 'credits-ex.wav') -Force
    Write-Host 'Build completed.' -ForegroundColor Green
    return './dist/credits-ex/credits-ex.exe'
}
catch {
    Write-Host "Build failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}
