# PowerShell Script Summary: Leptos 0.7 CSR Test Runner

This PowerShell script automates the testing and deployment process for a Leptos 0.7 CSR (Client-Side Rendering) application. It runs Rust and WebAssembly tests, verifies Tailwind CSS configuration, and starts a Trunk development server if all tests pass.

## Detailed Functionality

### Logging System
- Creates a timestamped log file (`test_run_YYYYMMDD_HHMMSS.log`)
- Uses the `Log-Output` function to write messages to both the console and log file
- Console output is color-coded for better readability
- All command outputs are captured in the log for detailed debugging

### Helper Functions
1. **Run-Command**
   - Executes commands with detailed logging
   - Captures and processes command output
   - Special handling for test output:
     - Extracts and displays failing tests
     - Shows test result summaries with color coding
   - Controls error handling with the `continueOnError` parameter

2. **Test-PortInUse**
   - Checks if a specific TCP port is already in use
   - Prevents port conflicts when starting the Trunk server

### Configuration Detection
1. **Trunk.toml Configuration**
   - Checks for the presence of a custom `Trunk.toml` file
   - Extracts the configured port (defaults to 8080 if not specified)
   - Logs the configuration details

2. **Tailwind CSS Configuration**
   - Detects whether Tailwind CSS is being used via CDN or local files
   - For local Tailwind setup:
     - Verifies the presence of required files (`tailwind.config.js`, `input.css`)
     - Notes optional files (`postcss.config.js`)
     - Tests Tailwind CSS compilation to ensure it works properly

### Testing Process
1. **Rust Unit Tests**
   - Runs standard Rust tests with `cargo test`
   - Displays test results with color-coded pass/fail status

2. **WebAssembly Tests**
   - Executes Wasm tests with `wasm-pack test --firefox --headless`
   - Shows test results with appropriate formatting

### Server Management
- Checks if the configured Trunk port is already in use
- If the port is available, starts the Trunk development server
- If the port is already in use, provides a warning and does not start the server

### Error Handling
- Detailed error logging to the log file
- Clear console feedback with color-coded messaging
- Graceful handling of configuration issues and test failures
- Process termination with appropriate exit codes when critical errors occur

This script provides a comprehensive automation solution for testing and running Leptos 0.7 CSR applications, with particular attention to proper Tailwind CSS integration and port management for the Trunk development server.