#!/bin/bash

echo "🦀 Building Rust library..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    source ~/.cargo/env
fi

# Clean previous build
echo "🧹 Cleaning previous build..."
rm -rf pkg/

# Build the WASM package for web target
echo "🚀 Building WASM package..."
wasm-pack build --target web --out-dir pkg

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "📁 Generated files in pkg/:"
    ls -la pkg/
    echo ""
    echo "🎯 Ready to use in web applications!"
else
    echo "❌ Build failed!"
    exit 1
fi