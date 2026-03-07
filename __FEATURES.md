# Missing Features vs shadcn CLI

Features that shadcn CLI has that our `ui-cli` currently lacks.


  ---
  shadcn CLI vs rust-ui CLI — Feature Comparison

  ┌─────────────────────────────────────────────────────────────────────────────┐
  │                         COMMAND COMPARISON                                  │
  ├─────────────────────┬───────────────────────┬────────────────────────────── │
  │  Feature            │  shadcn (TS)          │  rust-ui (Rust)               │
  ├─────────────────────┼───────────────────────┼───────────────────────────────┤
  │  init               │  ✅ Full              │  ✅ Basic                      │
  │  add                │  ✅ Full              │  ✅ Full + TUI                 │
  │  starters           │  ❌                   │  ✅ (tauri, tauri-fullstack)   │
  │  diff               │  ✅ (deprecated)      │  ✅                            │
  │  search             │  ✅                   │  ✅                            │
  │  list               │  ❌                   │  ✅ (rust-ui exclusive)        │
  │  update             │  ❌                   │  ✅ (rust-ui exclusive)        │
  │  info               │  ✅                   │  ✅                            │
  │  docs               │  ✅                   │  ✅                            │
  │  view               │  ✅                   │  ✅                            │
  │  migrate            │  ✅                   │  ❌ (not needed)               │
  │  build              │  ✅                   │  ❌ (not needed)               │
  │  mcp                │  ✅                   │  ❌                            │
  │  registry           │  ✅ (registry:add)    │  ❌ (not needed)               │
  └─────────────────────┴───────────────────────┴───────────────────────────────┘

  ---
  init — Both have it, shadcn's is far richer

  ┌─────────────────────────────────────────────┬────────────────────────────────────────────────┬───────────────────────┐
  │                 Capability                  │                     shadcn                     │        rust-ui        │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ Framework auto-detection                    │ ✅ (Next, Vite, Astro, Laravel, React Router…) │ ❌                    │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ Monorepo support                            │ ✅                                             │ ❌                    │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ Multiple templates                          │ ✅ 7 templates                                 │ ❌                    │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ Theme presets / color base selection        │ ✅                                             │ ❌                    │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ CSS variables toggle                        │ ✅                                             │ ❌                    │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ RTL support                                 │ ✅                                             │ ❌                    │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ Overwrite config prompt                     │ ✅                                             │ ❌ (TODO in TODOS.md) │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ Re-install existing components              │ ✅                                             │ ❌                    │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ Backup/restore on failure                   │ ✅                                             │ ❌                    │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ Leptos dep check                            │ ❌                                             │ ✅                    │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ Tailwind CSS file detection from Cargo.toml │ ❌                                             │ ✅                    │
  ├─────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
  │ merge package.json (preserves user deps)    │ ❌                                             │ ✅                    │
  └─────────────────────────────────────────────┴────────────────────────────────────────────────┴───────────────────────┘

  ---
  add — rust-ui has a unique interactive TUI (ratatui)

  ┌───────────────────────────────────┬────────────────────────────────┬──────────────────────────────────────────────────────────────────────────┐
  │            Capability             │             shadcn             │                                 rust-ui                                  │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ Add by name                       │ ✅                             │ ✅                                                                       │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ Dependency resolution             │ ✅                             │ ✅ (via tree_parser)                                                     │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ Interactive TUI picker            │ ❌ (simple multiselect prompt) │ ✅ Ratatui with tabs (components, hooks, blocks, icons, demos, settings) │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ Overwrite prompt                  │ ✅                             │ ❌ (TODO)                                                                │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ --all flag                        │ ✅                             │ ❌                                                                       │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ --dry-run                         │ ✅                             │ ✅                                                                       │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ --diff / --view                   │ ✅                             │ ❌                                                                       │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ --path override                   │ ✅                             │ ✅                                                                       │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ JS file dependency download       │ ❌                             │ ✅                                                                       │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ Cargo dep injection               │ ❌                             │ ✅                                                                       │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ Register module in lib.rs/main.rs │ ❌                             │ ✅                                                                       │
  ├───────────────────────────────────┼────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ Deprecated component warnings     │ ✅                             │ ❌                                                                       │
  └───────────────────────────────────┴────────────────────────────────┴──────────────────────────────────────────────────────────────────────────┘

  ---
  Commands only in shadcn (not needed / not planned for rust-ui)

  ┌──────────────┬───────────────────────────────────────────────────────────────────────────────────────────────────┐
  │   Command    │                                           What it does                                            │
  ├──────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────┤
  │ migrate      │ Run migrations: icons, radix, rtl — auto-refactors source files (not needed)                      │
  ├──────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────┤
  │ build        │ Build registry items from local source (for publishing custom registries) (not needed)            │
  ├──────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────┤
  │ mcp          │ Start an MCP server so AI tools (Claude, Cursor, VS Code, Codex) can call shadcn programmatically │
  ├──────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────┤
  │ registry add │ Add a registry source to the project config (not needed)                                          │
  └──────────────┴───────────────────────────────────────────────────────────────────────────────────────────────────┘


  

---

## Commands to Add

### `diff` (or `add --diff`)
Show a line-by-line diff between the locally installed component and the latest version in the registry.
- Without arguments: scan all installed components and report which ones have updates available
- With a component name: show the exact diff for that component
- `add --diff [path]` — inline diff during add
- `add --view [path]` — view registry file contents without installing

### `search` (alias: `list`)
Query items from the registry.
```
ui search <registry> --query <string> --limit <n> --offset <n>
```
- Accept registry names or URLs
- Filter by query string
- Paginate results (limit/offset)
- Output as JSON

### `migrate`
Run automated code migrations when the library evolves.
Available migrations in shadcn:
- `icons` — migrate UI components to a different icon library
- `rtl` — add RTL (right-to-left) support to components
- Accept a path/glob pattern to limit scope
- `--list` to enumerate available migrations
- `--yes` to skip confirmation

### `info`
Print a diagnostic summary of the current project:
- Detected framework and version
- Tailwind version and config path
- CSS variables / RTL / icon library settings
- All configured import aliases and resolved paths
- List of currently installed components
- Links to docs, component source, schema
- `--json` flag for machine-readable output

### `build` _(not planned)_
Build registry items from local source files so a custom registry can be published.
- Read local component source
- Validate against registry schema
- Output registry-compatible JSON files

### `mcp`
Expose the CLI as an MCP (Model Context Protocol) server so AI coding tools can call it programmatically.
- `ui mcp` — start the MCP stdio server
- `ui mcp init --client <client>` — write MCP config for a specific client
  - Supported clients: Claude Code, Cursor, VS Code, Codex, OpenCode
  - Merges config into the client's existing config file (`.mcp.json`, `.cursor/mcp.json`, etc.)

### `registry add`
Register a custom or third-party registry URL in the project config (`ui_config.toml`).

### `docs`
Open the rust-ui documentation in the default browser.

---

## Flags / Options to Add to Existing Commands

### `add` command
| Flag | Description |
|---|---|
| `--dry-run` | Preview which files would be written/overwritten without actually writing anything |
| `--overwrite` / `-o` | Overwrite existing files without prompting |
| `--yes` / `-y` | Skip all confirmation prompts (useful for CI/scripting) |
| `--all` / `-a` | Add all available components at once _(not planned)_ |
| `--path <path>` | Override the output directory for the component ✅ |
| `--silent` / `-s` | Suppress output |
| Overwrite prompt | When a component already exists, prompt the user before overwriting (noted in TODOS.md) |
| Deprecated component warnings | Warn when a requested component is deprecated |

### `init` command
| Flag / Behavior | Description |
|---|---|
| Overwrite prompt | Ask before overwriting existing `ui_config.toml` (noted in TODOS.md) |
| `--force` / `-f` | Force overwrite of existing config without prompting |
| `--yes` / `-y` | Skip all confirmation prompts |
| `--silent` / `-s` | Suppress output |
| Re-install existing components | Offer to re-download and overwrite already installed components |
| Backup/restore on failure | Back up config before writing; restore on unexpected exit |
| `--defaults` / `-d` | Use a default config without any prompts |

---

## Quality-of-Life Improvements

- **Installed component tracking** — know which components are installed at what version, to power `diff` and update detection
- **Monorepo awareness** — detect if running from a workspace root and guide the user to the right sub-package
- **`--json` output** on relevant commands for scripting/AI consumption
- **Custom registry support** — allow users to point `add`/`search` at a non-default registry URL
