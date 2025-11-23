# CLI Code Generator

A Rust-based tool that provides both a command-line interface and a web-based UI for generating standardized CLI applications with built-in support for version, help, and command dispatch patterns.

## Project Structure

```
cli-gen/
+-- crates/
|   +-- backend/              # Rust backend (Axum server + CLI tool)
|   |   +-- src/
|   |   |   +-- bin/
|   |   |   |   +-- cli_gen.rs    # CLI binary
|   |   |   +-- main.rs           # Web server binary
|   |   |   +-- lib.rs
|   |   |   +-- codegen/          # Code generation logic
|   |   |   +-- models/           # Data models
|   |   +-- static/               # Static assets (favicon, etc.)
|   |   +-- templates/            # Code generation templates
|   |   +-- index -> ../frontend/dist  # Symlink to frontend build
|   +-- frontend/             # Yew WASM frontend
|       +-- src/
|       |   +-- lib.rs            # Main app component
|       |   +-- models.rs
|       |   +-- components/       # UI components
|       +-- index.html            # Frontend HTML template
|       +-- dist/                 # Trunk build output (gitignored)
+-- docs/                     # Project documentation
+-- scripts/                  # Build and deployment scripts
|   +-- build.sh              # Build frontend and backend
|   +-- serve.sh              # Run development server
+-- work/                     # Generated CLI projects (gitignored)
+-- reference/                # Backup of old files (gitignored)
+-- Trunk.toml                # Trunk build configuration
+-- Cargo.toml                # Workspace configuration
```

## Prerequisites

- Rust (2024 edition)
- Trunk (for building the WASM frontend)
  ```bash
  cargo install trunk
  ```
- wasm32-unknown-unknown target
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

## Getting Started

### Option 1: Quick Start (Development)

Use the provided scripts to build and run the project:

```bash
# Build everything (frontend + backend)
./scripts/build.sh

# Run the development server
./scripts/serve.sh
```

The web UI will be available at http://localhost:3000

### Option 2: Manual Build

#### Build Frontend
```bash
trunk build
```

This will:
- Build the Yew WASM application
- Output to `crates/frontend/dist/`
- The backend symlinks to this directory

#### Build Backend
```bash
cargo build --release -p cli-codegen-backend
```

#### Run the Server
```bash
cd crates/backend
cargo run --bin cli-codegen-backend -- --port 3000
```

## Usage

### Web UI

1. Navigate to http://localhost:3000
2. Fill in the CLI configuration form:
   - Name
   - Short description
   - Long description
   - Copyright information
   - License type
   - Version support
   - Help support
3. Click "Generate" to create your CLI project
4. Generated files will be in `./work/<project-name>/`

### CLI Tool

Generate a CLI project from the command line:

```bash
cargo run -p cli-codegen-backend --bin cli_gen -- generate \
  --name my-cli \
  --short-desc "A brief description" \
  --long-desc "A longer description" \
  --author "Your Name" \
  --repository "https://github.com/yourusername/my-cli" \
  --license MIT \
  --output ./work/my-cli
```

After generation, setup and build the project:

```bash
cd work/my-cli
./scripts/setup.sh    # Initialize git and add sw-cli submodule
./scripts/build.sh    # Build the release binary
```

### Batch Processing

Generate from a JSON configuration file:

```bash
cargo run -p cli-codegen-backend --bin cli_gen -- batch \
  --file config.json \
  --output ./work/my-cli
```

## API Endpoints

The backend server exposes the following API endpoints:

- `GET /api/config` - Get default configuration
- `POST /api/config` - Update configuration
- `POST /api/generate` - Generate CLI code from configuration

Example API call:
```bash
curl -X POST http://localhost:3000/api/generate \
  -H "Content-Type: application/json" \
  -d '{
    "name": "my-cli",
    "short_description": "A brief description",
    "long_description": "A longer description",
    "copyright": "Copyright (c) 2025",
    "license": "MIT",
    "version_support": true,
    "help_support": true
  }'
```

## Generated Output

The tool generates the following files in `./work/<project-name>/`:

```
<project-name>/
├── COPYRIGHT                   # Copyright notice
├── Cargo.toml                 # Package manifest with sw-cli dependency
├── .gitmodules                # Git submodule configuration
├── .gitignore                 # Standard Rust .gitignore
├── build.rs                   # Build script using sw-cli macros
├── src/
│   ├── main.rs                # Entry point with clap Parser and sw_cli::version!()
│   ├── cli.rs                 # CLI argument definitions using clap derive
│   ├── lib.rs                 # Library module exports
│   ├── short-help.txt         # Brief help text
│   └── long-help.txt          # Detailed help text
├── scripts/
│   ├── setup.sh              # Initialize git and sw-cli submodule (executable)
│   └── build.sh              # Build with submodule update (executable)
└── lib/                      # Created by setup.sh
    └── sw-cli/               # Git submodule (initialized by setup.sh)
```

### sw-cli Integration

All generated CLIs use the [sw-cli](https://github.com/softwarewrighter/sw-cli) library for:
- **Version Information**: Automatic generation of version strings with build metadata
- **Help Text Management**: Separate files for short/long help loaded at build time
- **Build Metadata**: Git commit SHA, build timestamp, and hostname automatically included

The generated CLI follows the same pattern as [markdown-checker](https://github.com/softwarewrighter/markdown-checker).

## Architecture

- **Frontend**: Yew framework for WebAssembly, built with Trunk
- **Backend**: Axum web server with code generation capabilities
- **Templates**: Tera-based template system for code generation
- **Build System**: Cargo workspace with separate frontend/backend crates

## Development Notes

- The backend serves the frontend via a symlink: `crates/backend/index -> ../frontend/dist`
- All generated CLI projects are output to the `./work/` directory (gitignored)
- The `./reference/` directory contains backups of the old project structure (gitignored)
- Templates are located in `crates/backend/templates/`

## License

This project is licensed under the MIT License.
