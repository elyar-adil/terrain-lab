param(
    [string]$Focus = "4",
    [string]$Span = "0.35",
    [int]$Fps = 3,
    [int]$Budget = 9000,
    [string]$Name = "city.png",
    [int]$Width = 1280,
    [int]$Height = 800
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent $PSScriptRoot
$outputDirectory = Join-Path $projectRoot "docs\render-validation"
New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null

$viteEntry = Join-Path $projectRoot "node_modules\vite\bin\vite.js"
$vitePort = 4179
$alreadyRunning = $false
try { Invoke-WebRequest -UseBasicParsing "http://127.0.0.1:$vitePort/render-harness.html" | Out-Null; $alreadyRunning = $true } catch {}
$vite = $null
if (-not $alreadyRunning) {
    $vite = Start-Process -FilePath "node" -ArgumentList @($viteEntry, "--host", "127.0.0.1", "--port", "$vitePort", "--strictPort") -WorkingDirectory $projectRoot -WindowStyle Hidden -PassThru
    for ($attempt = 0; $attempt -lt 60; $attempt++) {
        try { Invoke-WebRequest -UseBasicParsing "http://127.0.0.1:$vitePort/render-harness.html" | Out-Null; break }
        catch { Start-Sleep -Milliseconds 250 }
    }
}

try {
    $edgeCandidates = @(
        "${env:ProgramFiles(x86)}\Microsoft\Edge\Application\msedge.exe",
        "$env:ProgramFiles\Microsoft\Edge\Application\msedge.exe"
    )
    $registeredEdge = (Get-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\msedge.exe" -ErrorAction SilentlyContinue)."(default)"
    if ($registeredEdge) { $edgeCandidates = @($registeredEdge) + $edgeCandidates }
    $edge = $edgeCandidates | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
    if (-not $edge) { throw "Microsoft Edge was not found" }
    $edgeProfile = Join-Path $projectRoot "target\headless-edge-profile"
    $output = Join-Path $outputDirectory $Name
    $url = "http://127.0.0.1:$vitePort/render-harness.html?mode=3d&focus=$Focus&span=$Span&fps=$Fps"
    $arguments = @(
        "--headless=new",
        "--user-data-dir=$edgeProfile",
        "--use-angle=swiftshader",
        "--enable-webgl",
        "--ignore-gpu-blocklist",
        "--hide-scrollbars",
        "--window-size=$Width,$Height",
        "--virtual-time-budget=$Budget",
        "--screenshot=$output",
        $url
    )
    $process = Start-Process -FilePath $edge -ArgumentList $arguments -WindowStyle Hidden -PassThru -Wait
    if ($process.ExitCode -ne 0 -or -not (Test-Path -LiteralPath $output)) { throw "Headless Edge failed" }
    Write-Output $output
} finally {
    if ($vite -and -not $vite.HasExited) { Stop-Process -Id $vite.Id -Force }
}
