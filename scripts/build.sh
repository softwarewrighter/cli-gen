#!/bin/bash

# Build script for CLI Code Generator
# Builds both backend and frontend crates

set -e

echo "Building CLI Code Generator..."

# Build the frontend (web UI) for WASM first
echo "Building frontend crate..."
if command -v trunk &> /dev/null; then
    echo "Building web UI with Trunk..."
    trunk build --release
    echo "Web UI build completed in crates/frontend/dist"
else
    echo "Trunk not found. Install with: cargo install trunk"
    exit 1
fi

# Ensure symlink exists from backend to frontend dist
echo "Ensuring symlink from backend/index to frontend/dist..."
cd crates/backend
if [ ! -L "index" ]; then
    ln -sf ../frontend/dist index
    echo "Created symlink backend/index -> frontend/dist"
fi
cd ../..

# Build the backend (server and CLI)
echo "Building backend crate..."
cargo build --release -p cli-codegen-backend

echo "Build completed successfully!"
echo "Run the server with: ./scripts/serve.sh"