# Check for Trunk.toml (optional file)
if (-not (Test-Path -Path "Trunk.toml")) {
    Write-Host "No Trunk.toml found. Will use default Trunk settings." -ForegroundColor Yellow
} 
else {
    Write-Host "Using custom Trunk.toml configuration." -ForegroundColor Green
}

# Check Tailwind configuration
$usingCDN = $false
if (Test-Path -Path "index.html") {
    $indexContent = Get-Content -Path "index.html" -Raw
    if ($indexContent -match "cdn\.tailwindcss\.com") {
        Write-Host "Using Tailwind CSS via CDN." -ForegroundColor Green
        $usingCDN = $true
    }
}

if (-not $usingCDN) {
    # Check for Tailwind config
    if (-not (Test-Path -Path "tailwind.config.js")) {
        Write-Host "tailwind.config.js not found! Tailwind CSS may not be properly configured." -ForegroundColor Red
        exit 1
    } 
    else {
        Write-Host "Tailwind CSS configuration found." -ForegroundColor Green
    }

    # Check for PostCSS config
    if (-not (Test-Path -Path "postcss.config.js")) {
        Write-Host "postcss.config.js not found! Tailwind CSS may not be properly configured." -ForegroundColor Yellow
    } 
    else {
        Write-Host "PostCSS configuration found." -ForegroundColor Green
    }

    # Check for CSS input file
    if (-not (Test-Path -Path "input.css")) {
        Write-Host "input.css not found! Tailwind CSS may not be properly configured." -ForegroundColor Red
        exit 1
    } 
    else {
        Write-Host "Tailwind CSS input file found." -ForegroundColor Green
    }

    # Test Tailwind CSS compilation
    Write-Host "Testing Tailwind CSS compilation..." -ForegroundColor Cyan
    try {
        npx tailwindcss -i input.css -o temp-output.css
        if (Test-Path -Path "temp-output.css") {
            Remove-Item -Path "temp-output.css"
            Write-Host "Tailwind CSS compilation successful." -ForegroundColor Green
        } else {
            Write-Host "Tailwind CSS compilation failed to produce output file." -ForegroundColor Red
            exit 1
        }
    } catch {
        Write-Host "Error running Tailwind CSS compilation: $_" -ForegroundColor Red
        exit 1
    }
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
wasm-pack test --firefox --headless
if ($LASTEXITCODE -ne 0) {
    Write-Host "Wasm tests failed!" -ForegroundColor Red
    exit 1
}
Write-Host "Wasm tests passed!" -ForegroundColor Green

Write-Host "All tests passed successfully!" -ForegroundColor Green
Write-Host "Starting Trunk development server..." -ForegroundColor Cyan

# Start the trunk server
trunk serve