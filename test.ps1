param(
    [switch]$All,
    [switch]$New,
    [switch]$Build,
    [switch]$Info,
    [switch]$Clean,
    [switch]$Rss
)

$ErrorActionPreference = "Stop"
Write-Host "=== Rust Blog Automation Pipeline Initialized ===" -ForegroundColor Cyan

if ($All -or $Clean) {
    Write-Host "Purging active cache registers and artifacts..." -ForegroundColor Yellow
    if (Test-Path "content") {
        Remove-Item "content\*" -Include *.md, *.markdown -Force -ErrorAction SilentlyContinue
    }
    if (Test-Path "public") {
        Remove-Item "public" -Recurse -Force -ErrorAction SilentlyContinue
    }
    if (Test-Path ".rust_blog_cache.bin") {
        Remove-Item ".rust_blog_cache.bin" -Force -ErrorAction SilentlyContinue
    }
}

Write-Host "Compiling workspace architecture binaries..." -ForegroundColor Green
cargo build

if ($LASTEXITCODE -ne 0) {
    Write-Host "Target compilation failure. Pipeline terminated." -ForegroundColor Red
    exit 1
}

$exe = ".\target\debug\forge-cli.exe"

if ($All -or $New) {
    Write-Host "`nRunning content generation sequence tests..." -ForegroundColor Yellow
    & $exe new "Rust Study Notes" --tags "rust,learning"
    & $exe new "Markdown Grammar Guide" --tags "markdown,tutorial"
    & $exe new "Draft Artifact Entry" --draft
}

if ($All -or $Info) {
    Write-Host "`nRetrieving system optimization diagnostics metrics..." -ForegroundColor Yellow
    & $exe info
}

if ($All -or $Build) {
    Write-Host "`nExecuting static distribution compilation pipeline..." -ForegroundColor Yellow
    & $exe build --output "public" --minify --gzip --incremental
    
    Write-Host "`nReviewing generated asset index layout mapping:" -ForegroundColor Cyan
    if (Test-Path "public") {
        Get-ChildItem "public" -Recurse | Where-Object { !$_.PSIsContainer } | Format-Table FullName, Length -AutoSize
    } else {
        Write-Host "Compilation failure: Directory public target matrix missing." -ForegroundColor Red
    }
}

if ($All -or $Rss) {
    Write-Host "`nEvaluating syndication RSS distribution engines..." -ForegroundColor Yellow
    & $exe rss --output "public/feed.xml"
}

Write-Host "`nAutomation Pipeline verification tasks finalized successfully!" -ForegroundColor Green