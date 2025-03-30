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

### 2. Testing and Running

The test scripts will automatically check your setup and start the development server if all tests pass:

**Windows:**
```
.\run_tests.ps1
```

**Linux/macOS:**
```
chmod +x run_tests.sh
./run_tests.sh
```

The scripts check for issues in this order:
1. Trunk.toml configuration file (will use defaults if not present)
2. Build errors (reported first if present)
3. Cargo test failures (standard Rust tests)
4. WebAssembly test failures (browser compatibility)

If all tests pass, you'll see "All tests passed successfully! 🎉" and the development server will automatically start at http://localhost:8080.

To manually start the server without running tests:

```bash
trunk serve
```

### 3. Development environment

This project uses:
- [Leptos](https://leptos.dev/) for reactive web UI
- [Tailwind CSS](https://tailwindcss.com/) for styling

### 4. Configuration (Optional)

Trunk works without configuration, but you can create a `Trunk.toml` file in your project root to customize its behavior:

```bash
# Create a basic Trunk.toml file
touch Trunk.toml
```

Example `Trunk.toml` configuration:

```toml
[serve]
# Address to serve on
address = "127.0.0.1"
# Port to serve on
port = 8080
# Open a browser tab when server launches
open = true
```

See the example Trunk.toml file for more configuration options.

## Project Structure

- `index.html`: Entry point for the web application
- `src/`: Source code directory
- `run_tests.ps1`/`run_tests.sh`: Test scripts for Windows and Unix systems

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License

Copyright (c) 2025 Ryan Khetlyr

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