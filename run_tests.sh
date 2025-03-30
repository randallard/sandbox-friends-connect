#!/bin/bash

# Check for Trunk.toml (optional file)
if [ ! -f "Trunk.toml" ]; then
    echo -e "\033[0;33mℹ️ No Trunk.toml found. Will use default Trunk settings.\033[0m"
else
    echo -e "\033[0;32m✅ Using custom Trunk.toml configuration.\033[0m"
fi

# Check Tailwind configuration
USING_CDN=false
if [ -f "index.html" ]; then
    if grep -q "cdn\.tailwindcss\.com" "index.html"; then
        echo -e "\033[0;32m✅ Using Tailwind CSS via CDN.\033[0m"
        USING_CDN=true
    fi
fi

if [ "$USING_CDN" = false ]; then
    # Check for Tailwind config
    if [ ! -f "tailwind.config.js" ]; then
        echo -e "\033[0;31m❌ tailwind.config.js not found! Tailwind CSS may not be properly configured.\033[0m"
        exit 1
    else
        echo -e "\033[0;32m✅ Tailwind CSS configuration found.\033[0m"
    fi

    # Check for PostCSS config
    if [ ! -f "postcss.config.js" ]; then
        echo -e "\033[0;33mℹ️ postcss.config.js not found! Tailwind CSS may not be properly configured.\033[0m"
    else
        echo -e "\033[0;32m✅ PostCSS configuration found.\033[0m"
    fi

    # Check for CSS input file
    if [ ! -f "input.css" ]; then
        echo -e "\033[0;31m❌ input.css not found! Tailwind CSS may not be properly configured.\033[0m"
        exit 1
    else
        echo -e "\033[0;32m✅ Tailwind CSS input file found.\033[0m"
    fi

    # Test Tailwind CSS compilation
    echo -e "\033[0;36mTesting Tailwind CSS compilation...\033[0m"
    if ! npx tailwindcss -i input.css -o temp-output.css; then
        echo -e "\033[0;31m❌ Tailwind CSS compilation failed!\033[0m"
        exit 1
    fi
    
    if [ -f "temp-output.css" ]; then
        rm temp-output.css
        echo -e "\033[0;32m✅ Tailwind CSS compilation successful.\033[0m"
    else
        echo -e "\033[0;31m❌ Tailwind CSS compilation failed to produce output file.\033[0m"
        exit 1
    fi
fi

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
echo -e "\033[0;36mStarting Trunk development server...\033[0m"

# Start the trunk server
trunk serve