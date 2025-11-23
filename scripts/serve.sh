#!/bin/bash

# Serve script for CLI Code Generator
# Serves the web UI locally for development

set -euo pipefail

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "${SCRIPT_DIR}/.." && pwd )"

cd "${PROJECT_ROOT}"

echo "Starting CLI Code Generator web server..."

# Build frontend with trunk if available
if command -v trunk &> /dev/null; then
    echo "Building web UI with Trunk..."
    trunk build
    echo "Web UI build completed in crates/frontend/dist"
else
    echo "Trunk not found. Install with: cargo install trunk"
    exit 1
fi

# Ensure symlink exists from backend to frontend dist
cd "${PROJECT_ROOT}/crates/backend"
if [ ! -L "index" ]; then
    ln -sf ../frontend/dist index
    echo "Created symlink backend/index -> frontend/dist"
fi

echo "Starting backend server (serves web UI and API)..."
cd "${PROJECT_ROOT}"
cargo run --bin cli-codegen-backend -- --port 3000