# Run standard Rust tests first
Write-Host "Running cargo tests..." -ForegroundColor Cyan
$cargoResult = cargo test
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Cargo tests failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Cargo tests passed!" -ForegroundColor Green

# If cargo tests pass, run wasm tests
Write-Host "Running wasm tests..." -ForegroundColor Cyan
$wasmResult = wasm-pack test --chrome --headless
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Wasm tests failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Wasm tests passed!" -ForegroundColor Green

Write-Host "All tests passed successfully! 🎉" -ForegroundColor Green