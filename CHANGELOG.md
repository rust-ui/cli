# rust-ui CLI — Changelog

## Unreleased

### Added

- `ui update` — checks all installed components against the registry; reports `up to date`, `outdated`, or `not in registry` per component; suggests the exact `ui add <name> -y` command to fix each; supports `--json`
- `ui search <query>` — filters the registry by name (case-insensitive); supports `--json` for scripted output
- `ui list --json` — machine-readable JSON output for `ui list` (`{ total, categories: { ... } }`)
- `ui list` — lists all available components from the registry grouped by category (grep-friendly, one component per line)
- `ui info --json` — machine-readable JSON output for `ui info`, useful for scripting and AI tooling
- `ui docs` — opens `https://rust-ui.com` in the system default browser (cross-platform: `open` / `xdg-open` / `start`)
- `ui add --dry-run` / `-n` — resolves all dependencies and previews which files would be written, overwritten, or skipped without touching the filesystem; output is sorted for determinism

---

## 0.3.7

### Added

- `ui info` — prints project config (`ui_config.toml`), base color, base path, workspace detection, and all installed components with count
- `ui add --yes` / `-y` — skips the overwrite prompt and forces all files to be written
- Overwrite prompt on `add` — when a component file already exists, the user is asked before overwriting (requires a TTY; bypassed with `--yes`)
- Explicit summary after `add`:
  - `✅ Added:` — newly written files
  - `⏭  Skipped:` — existing files the user chose not to overwrite
  - `📦 Dep already installed:` — auto-resolved dependency components already on disk (no silent skips)

### Changed

- Auto-resolved dependency components that are already installed no longer trigger the overwrite prompt; they are reported separately in the summary

---

## 0.3.6

### Added

- `ui add` reads tailwind input file path from `[package.metadata.leptos]` in `Cargo.toml`
- Workspace-aware Cargo dep injection: detects workspace root and uses `[workspace.dependencies]` when available

### Changed

- Removed deprecated starter templates
- Upgraded ratatui to 0.30

---

## 0.3.5

### Added

- JS file dependency support in `add`: downloads JS files to `public/` alongside Rust components

---

## 0.3.4

### Added

- Interactive TUI picker (ratatui) with tabs: Components, Hooks, Blocks, Icons, Demos, Settings
- Installed components highlighted in TUI list
- Dependency detail panel in TUI
- Footer keyboard shortcuts (`Ctrl+letter` to jump between tabs)
- Unit tests for TUI logic

---

## 0.1.5

### Added

- `ui starters` — choose and clone starter templates (Tauri, Tauri Fullstack)
- Registry-based component fetching (`add` reads from remote registry)
- Automatic `mod.rs` registration on `add`
- Automatic `pub mod components` registration in `lib.rs` / `main.rs`
- Cargo dependency injection on `add`

---

## 0.1.4 and earlier

- Initial `ui init` command: scaffolds `ui_config.toml`, installs Tailwind CSS config, wires Leptos dependencies
- Initial `ui add <components>` command: fetches components from registry by name with dependency resolution
- Workspace detection and multi-crate support
