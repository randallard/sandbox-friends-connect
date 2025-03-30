#!/bin/bash
# Run standard Rust tests first
echo -e "\033[0;36mRunning cargo tests...\033[0m"
if ! cargo test; then
    echo -e "\033[0;31m❌ Cargo tests failed!\033[0m"
    exit 1
fi
echo -e "\033[0;32m✅ Cargo tests passed!\033[0m"

# If cargo tests pass, run wasm tests
echo -e "\033[0;36mRunning wasm tests...\033[0m"
if ! wasm-pack test --chrome --headless; then
    echo -e "\033[0;31m❌ Wasm tests failed!\033[0m"
    exit 1
fi
echo -e "\033[0;32m✅ Wasm tests passed!\033[0m"

echo -e "\033[0;32mAll tests passed successfully! 🎉\033[0m"