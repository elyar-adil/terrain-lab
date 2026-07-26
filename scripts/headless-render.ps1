param(
    [ValidateRange(1, 30)]
    [int]$ValidationFps = 6,
    [switch]$FullTiming,
    [switch]$FullResolution
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent $PSScriptRoot
$fixturePath = Join-Path $projectRoot "public\render-fixture.json"
$outputDirectory = Join-Path $projectRoot "docs\render-validation"
New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null

# Terrain generation is intentionally compute-heavy. Running the fixture in an
# unoptimised Rust dev profile dominates this render test before Edge starts.
cargo run --quiet --release -p terrain-core --example render_fixture -- $fixturePath

$viteEntry = Join-Path $projectRoot "node_modules\vite\bin\vite.js"
$vite = Start-Process -FilePath "node" -ArgumentList @($viteEntry, "--host", "127.0.0.1", "--port", "4177", "--strictPort") -WorkingDirectory $projectRoot -WindowStyle Hidden -PassThru

try {
    $ready = $false
    for ($attempt = 0; $attempt -lt 50; $attempt++) {
        try {
            Invoke-WebRequest -UseBasicParsing "http://127.0.0.1:4177/render-harness.html" | Out-Null
            $ready = $true
            break
        } catch {
            Start-Sleep -Milliseconds 200
        }
    }
    if (-not $ready) { throw "Vite render harness did not start" }

    $edgeCandidates = @(
        "${env:ProgramFiles(x86)}\Microsoft\Edge\Application\msedge.exe",
        "$env:ProgramFiles\Microsoft\Edge\Application\msedge.exe"
    )
    $registeredEdge = (Get-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\msedge.exe" -ErrorAction SilentlyContinue)."(default)"
    if ($registeredEdge) { $edgeCandidates = @($registeredEdge) + $edgeCandidates }
    $edge = $edgeCandidates | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
    if (-not $edge) { throw "Microsoft Edge was not found" }
    $edgeProfile = Join-Path $projectRoot "target\headless-edge-profile"

    $earlyBudget = if ($FullTiming) { 3000 } else { 1200 }
    $lateBudget = if ($FullTiming) { 7000 } else { 3200 }
    $windowSize = if ($FullResolution) { "960,600" } else { "640,400" }
    $captures = @(
        @{ Name = "perspective-t06.png"; Mode = "3d"; Budget = $earlyBudget },
        @{ Name = "perspective-t12.png"; Mode = "3d"; Budget = $lateBudget },
        @{ Name = "satellite-t06.png"; Mode = "satellite"; Budget = $earlyBudget },
        @{ Name = "satellite-t12.png"; Mode = "satellite"; Budget = $lateBudget }
    )
    foreach ($capture in $captures) {
        $output = Join-Path $outputDirectory $capture.Name
        $url = "http://127.0.0.1:4177/render-harness.html?mode=$($capture.Mode)&fps=$ValidationFps"
        $arguments = @(
            "--headless=new",
            "--user-data-dir=$edgeProfile",
            "--use-angle=swiftshader",
            "--enable-webgl",
            "--ignore-gpu-blocklist",
            "--hide-scrollbars",
            "--window-size=$windowSize",
            "--virtual-time-budget=$($capture.Budget)",
            "--screenshot=$output",
            $url
        )
        $edgeProcess = Start-Process -FilePath $edge -ArgumentList $arguments -WindowStyle Hidden -PassThru -Wait
        if ($edgeProcess.ExitCode -ne 0 -or -not (Test-Path -LiteralPath $output)) {
            throw "Headless Edge failed to create $output"
        }
        Write-Output $output
    }
    cargo run --quiet --release -p terrain-core --example analyze_frames -- (Join-Path $outputDirectory "perspective-t06.png") (Join-Path $outputDirectory "perspective-t12.png")
    cargo run --quiet --release -p terrain-core --example analyze_frames -- (Join-Path $outputDirectory "satellite-t06.png") (Join-Path $outputDirectory "satellite-t12.png")
} finally {
    if ($vite -and -not $vite.HasExited) { Stop-Process -Id $vite.Id -Force }
    Remove-Item -LiteralPath $fixturePath -Force -ErrorAction SilentlyContinue
}
