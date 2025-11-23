# CLI Code Generator Architecture

## Overview
The CLI Code Generator is a Rust-based tool that provides both a command-line interface and a web-based user interface for generating standardized CLI applications. The architecture is designed to support rapid development of CLIs with consistent patterns for version, help, and command dispatch functionality.

All generated CLIs integrate with [sw-cli](https://github.com/softwarewrighter/sw-cli) for standardized version information, help text management, and build metadata.

## Components

### Core Engine
- **Code Generation Module**: Handles template rendering and file generation
- **Configuration Model**: Defines the structure of CLI configurations with sw-cli fields
- **Template System**: Manages templates for Rust source files, help text, and scripts
- **File Permissions**: Sets executable permissions on generated scripts

### Web Interface
- **Yew Frontend**: Single-page application built with Yew framework
- **Axum Backend**: Web server providing API endpoints for the frontend
- **Static Asset Serving**: Hosts the web UI and supporting assets

### CLI Interface
- **Command Handler**: Processes command-line arguments for generation
- **Batch Processing**: Supports configuration via JSON files
- **Code Generation Service**: Core service accessible via both CLI and web

## Generated Project Structure

```
<project-name>/
├── COPYRIGHT                   # Copyright notice (from config)
├── Cargo.toml                 # Package manifest with sw-cli dependency
├── .gitmodules                # Git submodule configuration for sw-cli
├── .gitignore                 # Standard Rust .gitignore
├── build.rs                   # Build script using sw-cli macros
├── src/
│   ├── main.rs                # Entry point with clap Parser and sw_cli::version!()
│   ├── cli.rs                 # CLI argument definitions using clap derive
│   ├── lib.rs                 # Library module exports
│   ├── short-help.txt         # Brief help text (loaded at build time)
│   └── long-help.txt          # Detailed help text (loaded at build time)
├── scripts/
│   ├── setup.sh              # Initialize git and sw-cli submodule (executable)
│   └── build.sh              # Build with submodule update (executable)
└── lib/                      # Created by setup.sh
    └── sw-cli/               # Git submodule (initialized by setup.sh)
```

## sw-cli Integration

### Build-Time Code Generation

Generated projects use sw-cli macros in `build.rs`:

```rust
fn main() {
    sw_cli::define_build_info!();    // Generates version metadata
    sw_cli::define_help_info!();     // Loads help text files
}
```

### Runtime Features

- **Version Information**: `sw_cli::version!()` provides formatted version with build metadata
- **Help Text**: `sw_cli::long_help!()` and `sw_cli::short_help!()` loaded from text files
- **Build Metadata**: Automatic inclusion of git commit, build time, and hostname

### Submodule Approach

sw-cli is integrated as a git submodule rather than a crates.io dependency:
- Allows use of latest features without waiting for publication
- Simplifies updates via `git submodule update --remote`
- Follows the pattern established by [markdown-checker](https://github.com/softwarewrighter/markdown-checker)

## Data Flow

1. User provides configuration (via web UI or CLI arguments)
   - Name, description, author, repository, license, version
2. Configuration is validated against schema
3. Template engine processes configuration:
   - Replaces `{{ name }}`, `{{ author }}`, etc. in templates
   - Generates crate_name from package name (replace `-` with `_`)
4. Generated files are written to output directory with proper structure
5. Scripts are made executable (755 permissions on Unix)

## Template System

### Template Variables

All templates support these replacements:
- `{{ name }}` - Package name (kebab-case)
- `{{ crate_name }}` - Crate name (snake_case, for use statements)
- `{{ short_description }}` - Brief CLI description
- `{{ long_description }}` - Detailed CLI description
- `{{ author }}` - Author name
- `{{ copyright }}` - Copyright notice
- `{{ license }}` - License type (MIT, Apache-2.0, GPL-3.0, etc.)
- `{{ repository }}` - Repository URL
- `{{ version }}` - Package version
- `{{ sw_cli_url }}` - URL to sw-cli repository

### Template Files

Located in `crates/backend/templates/`:
- `main.rs` - Main entry point
- `cli.rs` - CLI argument definitions
- `lib.rs` - Library exports
- `new_build.rs` - Build script (renamed to build.rs)
- `Cargo.toml` - Package manifest
- `COPYRIGHT` - Copyright notice
- `.gitmodules` - Submodule configuration
- `.gitignore` - Git ignore rules
- `short-help.txt` - Brief help
- `long-help.txt` - Detailed help
- `setup.sh` - Setup script
- `build.sh` - Build script

## Technology Stack
- **Rust** (2024 edition) - Core language
- **Yew** - Web framework for frontend
- **Axum** - Web server for backend
- **Serde** - Data serialization
- **Clap** - Used by generated CLIs for argument parsing
- **sw-cli** - Version and help text management for generated CLIs

## Post-Generation Workflow

1. **Generation**: `cli_gen generate --name my-cli ...`
2. **Setup**: `cd work/my-cli && ./scripts/setup.sh`
   - Initializes git repository
   - Adds sw-cli as submodule
   - Clones and initializes all submodules
3. **Build**: `./scripts/build.sh`
   - Updates submodules to latest
   - Runs `cargo build --release`
4. **Usage**: `./target/release/my-cli --version`

## References

- [sw-cli Integration Guide](./sw-cli-integration.md) - Detailed sw-cli usage
- [markdown-checker](https://github.com/softwarewrighter/markdown-checker) - Reference implementation