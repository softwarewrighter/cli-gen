#!/usr/bin/env bash
set -e

echo "Setting up {{ name }}..."
echo "======================"
echo

# Initialize git if not already a repo
if [ ! -d ".git" ]; then
    echo "Initializing git repository..."
    git init
fi

# Add sw-cli submodule
echo "Adding sw-cli submodule..."
if [ -d "lib/sw-cli" ]; then
    echo "sw-cli submodule already exists, skipping..."
else
    git submodule add {{ sw_cli_url }} lib/sw-cli
fi

# Initialize and update submodules
echo "Updating submodules..."
git submodule update --init --recursive

echo
echo "✓ Setup complete!"
echo
echo "Next steps:"
echo "  1. Review the generated code in src/"
echo "  2. Customize src/cli.rs with your CLI arguments"
echo "  3. Implement your logic in src/main.rs"
echo "  4. Run './scripts/build.sh' to build the project"
echo
