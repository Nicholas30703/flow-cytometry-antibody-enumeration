#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

echo "Building Rust to WebAssembly..."
wasm-pack build --target web --out-dir www/pkg

echo ""
echo "Build complete! The pkg/ directory has been generated."
echo ""
echo "To run the app, serve the www/ directory with any HTTP server:"
echo "  cd www && python -m http.server 8080"
echo "Then open http://localhost:8080 in your browser."
