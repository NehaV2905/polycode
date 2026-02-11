#!/bin/bash
set -e

echo "🦀 Module 2: IR Builder Setup Script"
echo "======================================"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed"
    echo ""
    echo "Please install Rust from: https://rustup.rs/"
    echo ""
    echo "Run this command:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
else
    echo "✅ Rust is installed: $(rustc --version)"
fi

# Check if protoc is installed
if ! command -v protoc &> /dev/null; then
    echo "⚠️  protoc (Protocol Buffers compiler) is not installed"
    echo ""
    echo "Install it with:"
    echo "  macOS:    brew install protobuf"
    echo "  Ubuntu:   apt-get install protobuf-compiler"
    echo "  Or download from: https://github.com/protocolbuffers/protobuf/releases"
    exit 1
else
    echo "✅ protoc is installed: $(protoc --version)"
fi

echo ""
echo "📦 Building Module 2..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "🧪 Running tests..."
    cargo test

    echo ""
    echo "✅ Setup complete!"
    echo ""
    echo "To run Module 2:"
    echo "  cargo run -- connect --file /path/to/file.py"
    echo ""
    echo "Make sure Module 1 is running first:"
    echo "  cd ../module1_adapter"
    echo "  python src/main.py --mode lsp --grpc-port 50051"
else
    echo "❌ Build failed"
    exit 1
fi
