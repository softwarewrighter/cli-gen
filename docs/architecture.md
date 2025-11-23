# CLI Code Generator Architecture

## Overview
The CLI Code Generator is a Rust-based tool that provides both a command-line interface and a web-based user interface for generating standardized CLI applications. The architecture is designed to support rapid development of CLIs with consistent patterns for version, help, and command dispatch functionality.

## Components

### Core Engine
- **Code Generation Module**: Handles template rendering and file generation
- **Configuration Model**: Defines the structure of CLI configurations
- **Template System**: Manages templates for different code components

### Web Interface
- **Yew Frontend**: Single-page application built with Yew framework
- **Axum Backend**: Web server providing API endpoints for the frontend
- **Static Asset Serving**: Hosts the web UI and supporting assets

### CLI Interface
- **Command Handler**: Processes command-line arguments
- **Batch Processing**: Supports configuration via script files
- **Code Generation Service**: Core service accessible via both CLI and web

## Data Flow

1. User provides configuration (via web UI or CLI arguments)
2. Configuration is validated against schema
3. Template engine processes configuration and generates code files
4. Generated files are written to specified output directory

## Technology Stack
- Rust (2024 edition) - Core language
- Yew - Web framework
- Axum - Web server
- Serde - Data serialization
- Tera - Template engine