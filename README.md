# Sandbox for Friends-Connect

A playground for developing and testing client functionality for the [friends-connect](https://github.com/randallard/friends-connect) API.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) for WebAssembly compilation
- [Trunk](https://trunkrs.dev/#install) for bundling and serving

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/friends-connect-sandbox.git
cd friends-connect-sandbox
```

### 2. Project Structure

Ensure your project structure looks like this:
```
project_root/
├── src/
│   ├── main.rs
│   ├── app.rs
│   └── test_utils.rs
├── dist/             # Directory for compiled assets
├── index.html
├── input.css
├── tailwind.config.js
├── postcss.config.js
├── Trunk.toml
├── Cargo.toml
└── ...
```

If the `dist` directory doesn't exist, create it:
```bash
mkdir -p dist
```

### 3. Tailwind CSS Setup

This project uses Tailwind CSS for styling. Make sure your configuration is correct:

1. **Update index.html**: Use the compiled CSS file instead of CDN
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Sandbox Friends Connect</title>
    <!-- Remove or comment out the CDN script if present -->
    <!-- <script src="https://cdn.tailwindcss.com"></script> -->
    
    <!-- Add this link instead -->
    <link data-trunk rel="css" href="dist/tailwind.css" />
  </head>
  <body>
    <link data-trunk rel="rust" data-wasm-opt="z" />
  </body>
</html>
```

2. **Update Trunk.toml**: Make the Tailwind compilation cross-platform
```toml
[serve]
address = "127.0.0.1"
port = 8080
open = true

[watch]
watch = ["src", "input.css", "index.html", "tailwind.config.js"]

[[hooks]]
stage = "pre_build"
# Cross-platform approach
command = "npx"
command_arguments = ["tailwindcss", "-i", "input.css", "-o", "dist/tailwind.css"]
```

### 4. Running Tests

You can run tests using the provided scripts or manually:

#### Using the test scripts:

**Windows:**
```
.\run_tests.ps1
```

**Linux/macOS:**
```
chmod +x run_tests.sh
./run_tests.sh
```

#### Running tests manually:

**Standard Rust tests:**
```bash
cargo test
```

**WebAssembly tests:**
```bash
# Run in Firefox (headless)
wasm-pack test --firefox --headless
```

### 5. Running the Development Server

To start the development server:

```bash
trunk serve
```

The application will be available at `http://localhost:8080` by default.

### 5. Development environment

This project uses:
- [Leptos](https://leptos.dev/) for reactive web UI
- [Tailwind CSS](https://tailwindcss.com/) for styling

## Common Issues and Solutions

### Cross-platform compatibility

For Windows users:
- Use `npx` instead of direct commands in Trunk.toml
- Make sure paths use forward slashes (/) even on Windows

For Unix users:
- Ensure shell scripts have execute permissions: `chmod +x *.sh`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License

Copyright (c) 2025-2029 Ryan Khetlyr

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.