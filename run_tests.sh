#!/bin/bash

# Leptos 0.7 CSR Test Runner
# This script automates testing and deployment for Leptos 0.7 CSR applications
# It runs Rust and WebAssembly tests, verifies Tailwind CSS configuration,
# and starts a Trunk development server if all tests pass.

# Set up variables
timestamp=$(date +"%Y%m%d_%H%M%S")
log_file="test_run_${timestamp}.log"
default_port=8080
trunk_port=$default_port

# ANSI color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create log file with header
echo "=== Leptos 0.7 CSR Test Run - $(date) ===" > "$log_file"
echo "" >> "$log_file"

# Function to log output to both console and log file
log_output() {
    local level=$1
    local message=$2
    local color=$NC
    
    case "$level" in
        "INFO") color=$BLUE ;;
        "SUCCESS") color=$GREEN ;;
        "WARNING") color=$YELLOW ;;
        "ERROR") color=$RED ;;
    esac
    
    # Output to console with color
    echo -e "${color}[$level] $message${NC}"
    
    # Output to log file without color codes
    echo "[$level] $message" >> "$log_file"
}

# Function to run a command with proper logging
run_command() {
    local command_name=$1
    local command=$2
    local continue_on_error=${3:-false}
    
    log_output "INFO" "Running $command_name..."
    echo "Command: $command" >> "$log_file"
    echo "----------------------------------------" >> "$log_file"
    
    # Run the command and capture output
    output=$(eval "$command" 2>&1)
    exit_code=$?
    
    # Log the output
    echo "$output" >> "$log_file"
    echo "----------------------------------------" >> "$log_file"
    echo "Exit code: $exit_code" >> "$log_file"
    echo "" >> "$log_file"
    
    # Process and display the output based on the command type
    if [[ "$command" == *"cargo test"* ]] || [[ "$command" == *"wasm-pack test"* ]]; then
        # Extract and display test results
        if echo "$output" | grep -q "test result: FAILED"; then
            # Find and display failing tests
            echo "$output" | grep -E "^test .+ ... FAILED" | while read -r line; do
                log_output "ERROR" "$line"
            done
            
            # Display test summary
            summary=$(echo "$output" | grep -E "test result: FAILED. [0-9]+ passed; [0-9]+ failed")
            log_output "ERROR" "$summary"
        elif echo "$output" | grep -q "test result: ok"; then
            # Display test summary for successful tests
            summary=$(echo "$output" | grep -E "test result: ok. [0-9]+ passed; [0-9]+ failed")
            log_output "SUCCESS" "$summary"
        else
            # For other outputs, show a few lines
            echo "$output" | tail -n 10 | while read -r line; do
                echo "$line"
            done
        fi
    else
        # For other commands, show the last few lines of output
        echo "$output" | tail -n 5 | while read -r line; do
            echo "$line"
        done
    fi
    
    # Check if we should continue on error
    if [ $exit_code -ne 0 ] && [ "$continue_on_error" = false ]; then
        log_output "ERROR" "$command_name failed with exit code $exit_code"
        log_output "INFO" "Check the log file for details: $log_file"
        exit $exit_code
    elif [ $exit_code -ne 0 ]; then
        log_output "WARNING" "$command_name completed with exit code $exit_code"
    else
        log_output "SUCCESS" "$command_name completed successfully"
    fi
    
    return $exit_code
}

# Function to check if a port is in use - cross-platform approach
test_port_in_use() {
    local port=$1
    
    # Try to create a TCP socket on the port
    # If it fails, the port is already in use
    if command -v nc &> /dev/null; then
        # Using netcat if available
        nc -z localhost "$port" >/dev/null 2>&1
        return $?
    elif command -v python3 &> /dev/null || command -v python &> /dev/null; then
        # Using Python as a fallback
        python_cmd="python"
        if command -v python3 &> /dev/null; then
            python_cmd="python3"
        fi
        
        $python_cmd -c "
import socket
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
try:
    s.bind(('127.0.0.1', $port))
    s.close()
    exit(1)  # Port is free
except socket.error:
    exit(0)  # Port is in use
" >/dev/null 2>&1
        return $?
    else
        # Last resort - try to use /dev/tcp on bash (might not work on all systems)
        (echo > /dev/tcp/localhost/$port) >/dev/null 2>&1
        if [ $? -eq 0 ]; then
            return 0  # Port is in use
        else
            return 1  # Port is free
        fi
    fi
}

# Function to detect trunk.toml configuration
detect_trunk_config() {
    log_output "INFO" "Checking for Trunk.toml configuration..."
    
    if [ -f "Trunk.toml" ]; then
        log_output "INFO" "Trunk.toml found, checking for port configuration..."
        
        # Try to extract port setting
        if grep -q "port" "Trunk.toml"; then
            trunk_port=$(grep -oP 'port\s*=\s*\K\d+' "Trunk.toml")
            log_output "INFO" "Custom port configured in Trunk.toml: $trunk_port"
        else
            log_output "INFO" "No custom port found in Trunk.toml, using default port: $default_port"
        fi
    else
        log_output "INFO" "No Trunk.toml found, using default port: $default_port"
    fi
}

# Function to detect Tailwind CSS configuration
detect_tailwind_config() {
    log_output "INFO" "Checking for Tailwind CSS configuration..."
    
    # Check for Tailwind CDN usage in index.html
    if [ -f "index.html" ] && grep -q "tailwindcss" "index.html"; then
        log_output "INFO" "Detected Tailwind CSS via CDN in index.html"
        return 0
    fi
    
    # Check for local Tailwind configuration
    if [ -f "tailwind.config.js" ]; then
        log_output "INFO" "Found tailwind.config.js"
        
        # Check for input.css
        if [ -f "input.css" ] || [ -f "src/input.css" ] || [ -f "styles/input.css" ]; then
            log_output "INFO" "Found Tailwind CSS input file"
            
            # Check for postcss.config.js (optional)
            if [ -f "postcss.config.js" ]; then
                log_output "INFO" "Found postcss.config.js"
            else
                log_output "INFO" "No postcss.config.js found (optional)"
            fi
            
            # Test Tailwind CSS compilation
            log_output "INFO" "Testing Tailwind CSS compilation..."
            if command -v npx &> /dev/null; then
                run_command "Tailwind CSS compilation test" "npx tailwindcss -i input.css -o /dev/null" true
            else
                log_output "WARNING" "npx not found, skipping Tailwind CSS compilation test"
            fi
            
            return 0
        else
            log_output "WARNING" "tailwind.config.js found but no input.css file detected"
            return 1
        fi
    fi
    
    log_output "INFO" "No Tailwind CSS configuration detected"
    return 0
}

# Main script execution

log_output "INFO" "Starting Leptos 0.7 CSR test runner..."

# Detect Trunk configuration
detect_trunk_config

# Detect Tailwind CSS configuration
detect_tailwind_config

# Run Rust unit tests
run_command "Rust unit tests" "cargo test"

# Run WebAssembly tests
run_command "WebAssembly tests" "wasm-pack test --firefox --headless"

# Check if port is available before starting Trunk server
log_output "INFO" "Checking if port $trunk_port is available..."
if test_port_in_use "$trunk_port"; then
    log_output "WARNING" "Port $trunk_port is already in use. Please close the application using this port or configure a different port in Trunk.toml"
else
    log_output "SUCCESS" "Port $trunk_port is available"
    log_output "INFO" "Starting Trunk server on port $trunk_port..."
    
    # Add a trap to handle CTRL+C more gracefully
    trap 'log_output "INFO" "Stopping Trunk server..."; echo ""; exit 0' INT
    
    # Run trunk server in foreground
    log_output "INFO" "Running 'trunk serve --open' (Press CTRL+C to stop)"
    trunk serve --open
fi

log_output "SUCCESS" "Test run completed. See $log_file for complete details."