#!/bin/bash

# Set up logging
LOG_FILE="test_run_$(date +"%Y%m%d_%H%M%S").log"
echo "Logging all output to $LOG_FILE"

# Helper function to log output
log_output() {
    local message="$1"
    local level="${2:-INFO}"
    local color=""
    
    # Set color based on level
    case "$level" in
        SUCCESS) color="\033[0;32m" ;;  # Green
        ERROR)   color="\033[0;31m" ;;  # Red
        WARNING) color="\033[0;33m" ;;  # Yellow
        PROGRESS) color="\033[0;36m" ;;  # Cyan
        INFO|*)  color="\033[0m" ;;      # Default
    esac
    
    # Write to console with minimal info
    echo -e "${color}${level}: ${message}\033[0m"
    
    # Write to log file with timestamp
    echo "[$(date +"%Y-%m-%d %H:%M:%S")] [${level}] ${message}" >> "$LOG_FILE"
}

# Helper function to run commands with logging
run_command() {
    local command="$1"
    local description="$2"
    local continue_on_error="${3:-false}"
    local is_test="${4:-false}"
    
    log_output "$description" "PROGRESS"
    
    # Log the command
    echo -e "\n>>> EXECUTING: $command\n" >> "$LOG_FILE"
    
    # Create a temporary file to capture output
    local temp_file=$(mktemp)
    
    # Run command and capture output to both log file and temp file
    if ! eval "$command" | tee -a "$LOG_FILE" "$temp_file" >/dev/null 2>&1; then
        # Command failed
        if [ "$is_test" = "true" ]; then
            # Extract failing test names based on the command type
            if [[ "$command" == *"cargo test"* ]]; then
                # Process cargo test output
                log_output "The following tests failed:" "ERROR"
                grep "test .* ... FAILED" "$temp_file" | sed -E 's/test (.*) ... FAILED/  - \1/' | while read -r line; do
                    log_output "$line" "ERROR"
                done
            elif [[ "$command" == *"wasm-pack test"* ]]; then
                # Process wasm-pack test output
                log_output "The following tests failed:" "ERROR"
                grep "FAIL.*" "$temp_file" | sed -E 's/FAIL.*?://g' | sed -E 's/\[.*?\]//g' | sed -E 's/^[ \t]+//g' | while read -r line; do
                    log_output "  - $line" "ERROR"
                done
            fi
        fi
        
        # Clean up temp file
        rm -f "$temp_file"
        
        if [ "$continue_on_error" = "true" ]; then
            log_output "$description completed with non-zero exit code" "WARNING"
            return 1
        else
            log_output "$description failed" "ERROR"
            log_output "See $LOG_FILE for details" "INFO"
            exit 1
        fi
    else
        # Clean up temp file
        rm -f "$temp_file"
        
        log_output "$description completed" "SUCCESS"
        return 0
    fi
}

# Initialize log file with header
cat > "$LOG_FILE" << EOF
======================================================
Leptos 0.7 CSR Test Runner
Started: $(date)
======================================================
EOF

# Check for Trunk.toml
if [ -f "Trunk.toml" ]; then
    log_output "Custom Trunk.toml configuration found" "SUCCESS"
    echo -e "\n>>> CONTENT OF Trunk.toml:\n" >> "$LOG_FILE"
    cat "Trunk.toml" >> "$LOG_FILE"
else
    log_output "No Trunk.toml found, using default settings" "INFO"
fi

# Check Tailwind configuration
USING_CDN=false
if [ -f "index.html" ]; then
    if grep -q "cdn\.tailwindcss\.com" "index.html"; then
        log_output "Tailwind CSS via CDN detected" "SUCCESS"
        USING_CDN=true
    fi
fi

if [ "$USING_CDN" = false ]; then
    # Check for Tailwind files
    CONFIG_OK=true
    
    # Required files
    for file in "tailwind.config.js" "input.css"; do
        if [ -f "$file" ]; then
            log_output "$file found" "SUCCESS"
            echo -e "\n>>> CONTENT OF $file:\n" >> "$LOG_FILE"
            cat "$file" >> "$LOG_FILE"
        else
            log_output "Required file $file is missing" "ERROR"
            CONFIG_OK=false
        fi
    done
    
    # Optional files
    if [ -f "postcss.config.js" ]; then
        log_output "postcss.config.js found" "SUCCESS"
        echo -e "\n>>> CONTENT OF postcss.config.js:\n" >> "$LOG_FILE"
        cat "postcss.config.js" >> "$LOG_FILE"
    else
        log_output "Optional file postcss.config.js is missing" "WARNING"
    fi
    
    if [ "$CONFIG_OK" = false ]; then
        log_output "Critical Tailwind CSS files are missing" "ERROR"
        exit 1
    fi
    
    # Test Tailwind CSS compilation
    run_command "npx tailwindcss -i input.css -o temp-output.css" "Testing Tailwind CSS compilation"
    
    if [ -f "temp-output.css" ]; then
        rm temp-output.css
    else
        log_output "Tailwind CSS compilation failed to produce output file" "ERROR"
        exit 1
    fi
fi

# Run standard Rust tests
run_command "cargo test" "Running Rust unit tests" "false" "true"

# Run wasm tests
run_command "wasm-pack test --firefox --headless" "Running WebAssembly tests" "false" "true"

log_output "All tests passed successfully! 🎉" "SUCCESS"
log_output "Starting Trunk development server..." "PROGRESS"

# Add final timestamp to log
echo -e "\n======================================================\nStarting Trunk server: $(date)\n======================================================" >> "$LOG_FILE"

# Start the trunk server
trunk serve