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

### 2. Testing

Testing your changes early helps identify issues in a clear progression:

**Windows:**
```
.\run_tests.ps1
```

**Linux/macOS:**
```
chmod +x run_tests.sh
./run_tests.sh
```

The test scripts will check for issues in this order:
1. Build errors (reported first if present)
2. Cargo test failures (standard Rust tests)
3. WebAssembly test failures (browser compatibility)

If all tests pass, you'll see "All tests passed successfully! 🎉"

### 3. Running the development server

Once tests are passing, start the local development server:

```bash
trunk serve
```

This will compile and serve the application, typically at http://localhost:8080

### 4. Development environment

This project uses:
- [Leptos](https://leptos.dev/) for reactive web UI
- [Tailwind CSS](https://tailwindcss.com/) for styling

## Project Structure

- `index.html`: Entry point for the web application
- `src/`: Source code directory
- `run_tests.ps1`/`run_tests.sh`: Test scripts for Windows and Unix systems

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License

Copyright (c) 2025 [Your Name]

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