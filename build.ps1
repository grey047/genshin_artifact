#!/usr/bin/env pwsh
# Skirk / mona_core 全量构建脚本
# 用法: .\build.ps1 [-SkipCheck] [-SkipWasm] [-SkipMeta]

param(
    [switch]$SkipCheck,
    [switch]$SkipWasm,
    [switch]$SkipMeta
)

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot

Write-Host "=== genshin_artifact build ===" -ForegroundColor Cyan

# Step 1: cargo check
if (-not $SkipCheck) {
    Write-Host "`n[1/3] cargo check ..." -ForegroundColor Yellow
    Push-Location "$root\mona_core"
    $checkOutput = cmd /c "cargo check 2>&1"
    $checkExit = $LASTEXITCODE
    foreach ($line in $checkOutput) {
        if ($line -match "^error") { Write-Host $line -ForegroundColor Red }
        elseif ($line -match "^warning") { Write-Host $line -ForegroundColor DarkYellow }
        else { Write-Host $line }
    }
    if ($checkExit -ne 0 -and ($checkOutput | Select-String "^error").Count -gt 0) {
        Write-Host "cargo check failed with errors, aborting." -ForegroundColor Red
        Pop-Location
        exit 1
    }
    Pop-Location
    Write-Host "[1/3] cargo check OK" -ForegroundColor Green
} else {
    Write-Host "[1/3] cargo check SKIPPED" -ForegroundColor DarkGray
}

# Step 2: gen_meta (regenerate _gen_*.js + i18n)
if (-not $SkipMeta) {
    Write-Host "`n[2/3] gen_meta ..." -ForegroundColor Yellow
    Push-Location $root
    npm run gen_meta
    if ($LASTEXITCODE -ne 0) {
        Write-Host "gen_meta failed, aborting." -ForegroundColor Red
        Pop-Location
        exit 1
    }
    Pop-Location
    Write-Host "[2/3] gen_meta OK" -ForegroundColor Green
} else {
    Write-Host "[2/3] gen_meta SKIPPED" -ForegroundColor DarkGray
}

# Step 3: build:wasm
if (-not $SkipWasm) {
    Write-Host "`n[3/3] build:wasm ..." -ForegroundColor Yellow
    Push-Location $root
    npm run build:wasm
    if ($LASTEXITCODE -ne 0) {
        Write-Host "build:wasm failed, aborting." -ForegroundColor Red
        Pop-Location
        exit 1
    }
    Pop-Location
    Write-Host "[3/3] build:wasm OK" -ForegroundColor Green
} else {
    Write-Host "[3/3] build:wasm SKIPPED" -ForegroundColor DarkGray
}

Write-Host "`n=== Build complete ===" -ForegroundColor Cyan
