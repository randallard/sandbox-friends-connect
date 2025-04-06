# Helper function to run commands with logging
function Run-Command {
    param (
        [string]$command,
        [string]$description,
        [switch]$continueOnError,
        [switch]$isTest
    )
    
    Log-Output "PROGRESS: $description" "Cyan"
    
    # Add the command to the log file
    Add-Content -Path $logFile -Value "`n>>> EXECUTING: $command`n"
    
    # Create a temporary file to capture output
    $tempFile = [System.IO.Path]::GetTempFileName()
    
    # Run the command and capture output to temp file only (not to console)
    Invoke-Expression -Command "$command 2>&1" | Out-File -FilePath $tempFile
    
    # Add the output to the log file (but not to console)
    $output = Get-Content -Path $tempFile -Raw
    Add-Content -Path $logFile -Value $output
    
    # Check if the command succeeded
    if ($LASTEXITCODE -ne 0) {
        if ($isTest) {
            # For test commands, extract just what we want to show
            $testOutput = Get-Content -Path $tempFile
            
            # 1. Show failing test lines - use a more flexible pattern
            $failingTests = $testOutput | Select-String -Pattern "test .* \.\.\. FAIL(ED)?" | ForEach-Object { $_.Line }
            
            if ($failingTests.Count -gt 0) {
                Write-Host "The following tests failed:" -ForegroundColor Red
                foreach ($test in $failingTests) {
                    Write-Host "  - $test" -ForegroundColor Red
                }
            }
            
            # 2. Show test result summary
            $testSummary = $testOutput | Select-String -Pattern "test result:.*" | Select-Object -First 1
            if ($testSummary) {
                $summaryText = $testSummary.ToString()
                
                # Colorize the test result summary
                if ($summaryText -match "(test result:) (FAILED|PASSED)(.*)") {
                    $prefix = $matches[1]
                    $result = $matches[2]
                    $suffix = $matches[3]
                    
                    # Write prefix in cyan
                    Write-Host $prefix -ForegroundColor Cyan -NoNewline
                    
                    # Write result in red (FAILED) or green (PASSED)
                    if ($result -eq "FAILED") {
                        Write-Host " $result" -ForegroundColor Red -NoNewline
                    } else {
                        Write-Host " $result" -ForegroundColor Green -NoNewline
                    }
                    
                    # Write the rest in cyan
                    Write-Host $suffix -ForegroundColor Cyan
                }
            }
        }
        else {
            # For build errors, show the key error messages
            $buildOutput = Get-Content -Path $tempFile
            
            # Look for error messages
            $errorMessages = $buildOutput | Select-String -Pattern "error(\[E\d+\])?:"
            
            if ($errorMessages.Count -gt 0) {
                Write-Host "Build errors detected:" -ForegroundColor Red
                
                # Process each error to extract relevant parts
                foreach ($error in $errorMessages) {
                    # Display each error line
                    Write-Host "  $($error.Line)" -ForegroundColor Red
                }
                
                # Also check for "could not compile" messages which give summary info
                $compileErrors = $buildOutput | Select-String -Pattern "error: could not compile"
                foreach ($error in $compileErrors) {
                    Write-Host "  $($error.Line)" -ForegroundColor Red
                }
            }
        }
        
        # Remove temp file
        Remove-Item -Path $tempFile
        
        if (-not $continueOnError) {
            # Just indicate where full details can be found, without extra error messages
            Log-Output "See $logFile for full error details" "Yellow"
            exit 1
        } else {
            return $false
        }
    }
    else {
        if ($isTest) {
            # For successful tests, still show the summary
            $testOutput = Get-Content -Path $tempFile
            
            # Display all failed tests - including when overall process succeeded but individual tests failed
            $failingTests = $testOutput | Select-String -Pattern "test .* \.\.\.\ FAIL" | ForEach-Object { $_.Line }
            if ($failingTests.Count -gt 0) {
                Write-Host "The following tests failed:" -ForegroundColor Red
                foreach ($test in $failingTests) {
                    Write-Host "  - $test" -ForegroundColor Red
                }
            }
            
            $testSummary = $testOutput | Select-String -Pattern "test result:.*" | Select-Object -First 1
            
            if ($testSummary) {
                $summaryText = $testSummary.ToString()
                
                # Colorize the test result summary
                if ($summaryText -match "(test result:) (FAILED|PASSED)(.*)") {
                    $prefix = $matches[1]
                    $result = $matches[2]
                    $suffix = $matches[3]
                    
                    # Write prefix in cyan
                    Write-Host $prefix -ForegroundColor Cyan -NoNewline
                    
                    # Write result in green (PASSED)
                    Write-Host " $result" -ForegroundColor Green -NoNewline
                    
                    # Write the rest in cyan
                    Write-Host $suffix -ForegroundColor Cyan
                }
            } else {
                # If we can't find a test summary but the command succeeded,
                # print an explicit success message
                Log-Output "Rust unit tests passed successfully!" "Green"
            }
        }
        else {
            # For non-test commands that succeeded, we might still want to show warnings
            $buildOutput = Get-Content -Path $tempFile
            $warningMessages = $buildOutput | Select-String -Pattern "warning:"
            
            if ($warningMessages.Count -gt 0) {
                Write-Host "Warnings detected (but build succeeded):" -ForegroundColor Yellow
                Write-Host "  $($warningMessages.Count) warnings found - see log for details" -ForegroundColor Yellow
            }
        }
        
        # Remove temp file
        Remove-Item -Path $tempFile
        
        Log-Output "SUCCESS: $description completed" "Green"
        return $true
    }
}

# Helper function to check if a port is in use
function Test-PortInUse {
    param (
        [int]$port
    )
    
    $connection = New-Object System.Net.Sockets.TcpClient
    
    try {
        $connection.Connect("localhost", $port)
        $connection.Close()
        return $true
    }
    catch {
        return $false
    }
}

# Set up logging
$logFile = "test_run_$(Get-Date -Format 'yyyyMMdd_HHmmss').log"
Write-Host "Logging all output to $logFile" -ForegroundColor Cyan

# Helper function to log output
function Log-Output {
    param (
        [string]$message,
        [string]$color = "White"
    )
    
    # Write to console with minimal info
    Write-Host $message -ForegroundColor $color
    
    # Write detailed message to log file
    Add-Content -Path $logFile -Value $message
}

# Initialize log file with header
$headerText = @"
======================================================
Leptos 0.7 CSR Test Runner
Started: $(Get-Date)
======================================================
"@
Add-Content -Path $logFile -Value $headerText

# Check for Trunk.toml
if (Test-Path -Path "Trunk.toml") {
    Log-Output "FOUND: Custom Trunk.toml configuration" "Green"
    Add-Content -Path $logFile -Value (Get-Content -Path "Trunk.toml" -Raw)
    
    # Extract port from Trunk.toml if it exists
    $trunkConfig = Get-Content -Path "Trunk.toml" -Raw
    if ($trunkConfig -match '(?m)^\s*port\s*=\s*(\d+)') {
        $trunkPort = [int]$matches[1]
        Log-Output "INFO: Found port $trunkPort in Trunk.toml" "Cyan"
    } else {
        $trunkPort = 8080 # Default Trunk port
        Log-Output "INFO: Using default Trunk port 8080" "Cyan"
    }
} 
else {
    Log-Output "INFO: No Trunk.toml found, using default settings" "Yellow"
    $trunkPort = 8080 # Default Trunk port
    Log-Output "INFO: Using default Trunk port 8080" "Cyan"
}

# Check Tailwind configuration
$usingCDN = $false
if (Test-Path -Path "index.html") {
    $indexContent = Get-Content -Path "index.html" -Raw
    if ($indexContent -match "cdn\.tailwindcss\.com") {
        Log-Output "FOUND: Tailwind CSS via CDN" "Green"
        $usingCDN = $true
    }
}

if (-not $usingCDN) {
    # Check for Tailwind files
    $tailwindFiles = @{
        "tailwind.config.js" = $true
        "postcss.config.js" = $false  # Optional
        "input.css" = $true
    }
    
    $configOk = $true
    
    foreach ($file in $tailwindFiles.Keys) {
        $required = $tailwindFiles[$file]
        if (Test-Path -Path $file) {
            Log-Output "FOUND: $file" "Green"
            Add-Content -Path $logFile -Value ("`n>>> CONTENT OF " + $file + ":`n")
            Add-Content -Path $logFile -Value (Get-Content -Path $file -Raw)
        }
        elseif ($required) {
            Log-Output "ERROR: Required file $file is missing" "Red"
            $configOk = $false
        }
        else {
            Log-Output "WARNING: Optional file $file is missing" "Yellow"
        }
    }
    
    if (-not $configOk) {
        Log-Output "ERROR: Critical Tailwind CSS files are missing" "Red"
        exit 1
    }
    
    # Test Tailwind CSS compilation
    Run-Command -command "npx tailwindcss -i input.css -o temp-output.css" -description "Testing Tailwind CSS compilation"
    
    if (Test-Path -Path "temp-output.css") {
        Remove-Item -Path "temp-output.css"
    } else {
        Log-Output "ERROR: Tailwind CSS compilation failed to produce output file" "Red"
        exit 1
    }
}

# Run standard Rust tests
$testResult = Run-Command -command "cargo test" -description "Running Rust unit tests" -isTest -continueOnError

if (-not $testResult) {
    # When tests fail, we still want to attempt the WASM tests
    Log-Output "WARNING: Rust unit tests failed, but continuing with WASM tests" "Yellow"
}

# Run wasm tests
$wasmTestResult = Run-Command -command "wasm-pack test --firefox --headless" -description "Running WebAssembly tests" -isTest -continueOnError

if ($testResult -and $wasmTestResult) {
    Log-Output "SUCCESS: All tests passed successfully! 🎉" "Green"
} else {
    if (-not $testResult) {
        Log-Output "ERROR: Rust unit tests failed" "Red"
    }
    if (-not $wasmTestResult) {
        Log-Output "ERROR: WASM tests failed" "Red"
    }
}

# Check if port is already in use
if (Test-PortInUse -port $trunkPort) {
    Log-Output "WARNING: Port $trunkPort is already in use!" "Yellow"
    Log-Output "INFO: Trunk server will not be started to avoid conflicts." "Yellow"
    Log-Output "INFO: Please check for other instances of Trunk or services using port $trunkPort" "Yellow"
    
    # Add final timestamp to log
    Add-Content -Path $logFile -Value "`n======================================================`nPort $trunkPort already in use - Trunk server not started: $(Get-Date)`n======================================================"
} else {
    # Only start Trunk if both tests passed
    if ($testResult -and $wasmTestResult) {
        Log-Output "PROGRESS: Starting Trunk development server on port $trunkPort..." "Cyan"
        
        # Add final timestamp to log
        Add-Content -Path $logFile -Value "`n======================================================`nStarting Trunk server: $(Get-Date)`n======================================================"
        
        # Start the trunk server
        trunk serve
    } else {
        Log-Output "WARNING: Not starting Trunk server due to test failures" "Yellow"
        Log-Output "INFO: Fix the errors above before starting the server" "Yellow"
        
        # Add final timestamp to log
        Add-Content -Path $logFile -Value "`n======================================================`nTrunk server not started due to test failures: $(Get-Date)`n======================================================"
    }
}