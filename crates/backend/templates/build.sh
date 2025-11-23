#!/usr/bin/env bash
set -e

echo "Building {{ name }}..."
echo "===================="
echo

# Always ensure submodules are initialized and updated to latest
echo "Updating git submodules to latest..."
git submodule update --init --recursive --remote --merge

echo
echo "Building release binary..."
cargo build --release

echo
echo "✓ Build completed successfully!"
echo
echo "Binary location: ./target/release/{{ name }}"
echo
