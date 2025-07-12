# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a CLI tool (`ui-cli`) for adding UI components to Rust/Leptos applications. The project consists of:

- **Main CLI Binary**: Located in `/crates/ui-cli/` - A Rust CLI tool for managing UI components
- **Component Registry**: A system for fetching and installing UI components from a remote registry
- **Starter Templates**: Pre-configured project templates for different Leptos setups
- **TailwindCSS Integration**: Built-in support for TailwindCSS v4

## Development Commands

### Building and Testing
```bash
# Build the CLI tool
cd crates/ui-cli
cargo build

# Run with specific commands (examples from main.rs)
cargo run --bin ui init
cargo run --bin ui add button demo_button demo_button_variants demo_button_sizes
cargo run --bin ui add demo_use_floating_placement
cargo run --bin ui starters

# Check code quality
cargo check
cargo clippy
cargo fmt

# Install frontend dependencies
pnpm install

# Build TailwindCSS
npx @tailwindcss/cli@next -i style/tailwind.css -o style/output.css --watch
```

### Running the CLI
The binary is named `ui` and supports three main commands:
- `ui init` - Initialize project with necessary config files
- `ui add [components...]` - Add UI components from registry
- `ui starters` - Clone starter template repositories

## Architecture

### CLI Command Structure
- **Command Init** (`src/command_init/`): Handles project initialization
  - Creates `ui_config.toml`, `package.json`, TailwindCSS config
  - Sets up project structure and dependencies
- **Command Add** (`src/command_add/`): Manages component installation
  - Fetches components from remote registry
  - Resolves dependencies automatically
  - Updates Cargo.toml and mod.rs files
- **Command Starters** (`src/command_starters/`): Clones starter templates
  - Supports trunk, leptos-ssr, leptos-ssr-workspace templates

### Key Components
- **Registry System**: Fetches components from remote JSON registry at runtime
- **Dependency Resolution**: Automatically resolves component dependencies and cargo crates
- **File Management**: Creates and updates mod.rs files and Cargo.toml entries
- **Configuration**: Uses `ui_config.toml` for project-specific settings

### Project Structure
```
crates/ui-cli/
├── src/
│   ├── command_add/          # Component installation logic
│   ├── command_init/         # Project initialization
│   ├── command_starters/     # Starter template cloning
│   ├── constants/            # CLI constants and URLs
│   └── shared/               # Shared utilities
└── Cargo.toml               # Binary configuration
```

## Configuration Files

### UI Config (`ui_config.toml`)
Contains project-specific settings like component base paths and TailwindCSS input file location.

### Code Quality
- Strict Clippy lints enforced (see main.rs deny attributes)
- No `unwrap()`, `panic!`, `todo!`, or indexing allowed in non-test code
- Rustfmt with max_width = 120

## Registry System

Components are fetched from a remote registry at runtime. The system:
1. Fetches `index.json` from remote registry
2. Resolves component dependencies 
3. Downloads component files
4. Updates local project files (mod.rs, Cargo.toml)
5. Creates directory structure as needed

## Working with Dependencies

When adding components, the CLI automatically:
- Resolves dependency trees for components
- Updates Cargo.toml with required crates
- Creates/updates mod.rs files with new modules
- Registers components in the main application entry point