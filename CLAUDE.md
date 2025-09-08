# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building and Testing
```bash
# Run with specific commands (examples from main.rs)
cargo run --bin ui init
cargo run --bin ui add button demo_button demo_button_variants demo_button_sizes
cargo run --bin ui add demo_use_floating_placement
cargo run --bin ui starters
```


### Project Structure
```
crates/ui-cli/
├── src/
│   ├── command_add/          # Component installation logic
│   ├── command_init/         # Project initialization
│   ├── command_starters/     # Starter template cloning
│   └── shared/               # Shared utilities
└── Cargo.toml               # Binary configuration
```

