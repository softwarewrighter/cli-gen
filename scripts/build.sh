#!/bin/bash

# Build script for CLI Code Generator
# Builds both backend and frontend crates

set -euo pipefail

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "${SCRIPT_DIR}/.." && pwd )"

cd "${PROJECT_ROOT}"

echo "Building CLI Code Generator..."

# Update cache-busting timestamp for favicon
echo "Updating favicon cache-busting timestamp..."
TIMESTAMP=$(($(date +%s) * 1000))
sed -i.bak "s/favicon\.ico?ts=[0-9]*[^\"]*\"/favicon.ico?ts=${TIMESTAMP}\"/" crates/frontend/index.html
rm -f crates/frontend/index.html.bak
echo "Updated favicon timestamp to: ${TIMESTAMP}"

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
cd "${PROJECT_ROOT}/crates/backend"
if [ ! -L "index" ]; then
    ln -sf ../frontend/dist index
    echo "Created symlink backend/index -> frontend/dist"
fi

# Build the backend (server and CLI)
echo "Building backend crate..."
cd "${PROJECT_ROOT}"
cargo build --release -p cli-codegen-backend

echo "Build completed successfully!"
echo "Run the server with: ./scripts/serve.sh"