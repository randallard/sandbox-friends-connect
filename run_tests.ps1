# Check for Trunk.toml (optional file)
if (-not (Test-Path -Path "Trunk.toml")) {
    Write-Host "No Trunk.toml found. Will use default Trunk settings." -ForegroundColor Yellow
} 
else {
    Write-Host "Using custom Trunk.toml configuration." -ForegroundColor Green
}

# Run standard Rust tests first
Write-Host "Running cargo tests..." -ForegroundColor Cyan
cargo test
if ($LASTEXITCODE -ne 0) {
    Write-Host "Cargo tests failed!" -ForegroundColor Red
    exit 1
}
Write-Host "Cargo tests passed!" -ForegroundColor Green

# If cargo tests pass, run wasm tests
Write-Host "Running wasm tests..." -ForegroundColor Cyan
wasm-pack test --chrome --headless
if ($LASTEXITCODE -ne 0) {
    Write-Host "Wasm tests failed!" -ForegroundColor Red
    exit 1
}
Write-Host "Wasm tests passed!" -ForegroundColor Green

Write-Host "All tests passed successfully!" -ForegroundColor Green
Write-Host "Starting Trunk development server..." -ForegroundColor Cyan

# Start the trunk server
trunk serve