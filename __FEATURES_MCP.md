# MCP Server for rust-ui CLI

## What is MCP?

Model Context Protocol (MCP) is an open protocol that lets AI assistants (Claude, Cursor, VS Code Copilot, etc.)
call external tools. An MCP server exposes named tools with typed inputs; the AI decides when to call them
and what to pass. Communication happens over **stdio** (stdin/stdout), so any binary can be an MCP server
— including our CLI.

---

## How Shadcn Implements It (Reference)

### Architecture

The CLI binary IS the MCP server. Running `shadcn mcp` connects to stdin/stdout via `StdioServerTransport`
from `@modelcontextprotocol/sdk`. No separate process, no HTTP, no daemon.

```
AI Client (Claude Code)
        |
        | stdin/stdout (JSON-RPC 2.0)
        v
  shadcn mcp  ←→  registry HTTP API
```

The `mcp init --client <X>` subcommand writes the editor config file so the client knows how to launch the server.

### CLI commands

```
shadcn mcp                     # starts stdio server
shadcn mcp init --client X     # writes .mcp.json / .cursor/mcp.json / etc.
```

### 7 Tools Exposed

| Tool | Purpose | Inputs |
|---|---|---|
| `get_project_registries` | Read registry names from `components.json` | none |
| `list_items_in_registries` | List all items with pagination | `registries[]`, `limit?`, `offset?` |
| `search_items_in_registries` | Fuzzy search across registries | `registries[]`, `query`, `limit?`, `offset?` |
| `view_items_in_registries` | View full file contents of items | `items[]` (`@reg/name` format) |
| `get_item_examples_from_registries` | Find demo/usage code | `registries[]`, `query` |
| `get_add_command_for_items` | Returns the `npx shadcn add ...` command | `items[]` |
| `get_audit_checklist` | Post-generation checklist (static text) | none |

### Editor Config Files Written

| Client | File |
|---|---|
| Claude Code | `.mcp.json` |
| Cursor | `.cursor/mcp.json` |
| VS Code | `.vscode/mcp.json` |
| OpenCode | `opencode.json` |
| Codex | `~/.codex/config.toml` (manual — CLI cannot write global files) |

### Config format (Claude Code `.mcp.json`)

```json
{
  "mcpServers": {
    "shadcn": {
      "command": "npx",
      "args": ["shadcn@latest", "mcp"]
    }
  }
}
```

The AI client reads this file, launches the command, and pipes stdio to it.

### Key design decisions

- **No network in the server loop** — registry HTTP calls happen only when a tool is actually invoked.
- **Stateless** — no session state; each tool call reads config fresh from disk (`components.json`).
- **Registry-aware** — tools pass registry names explicitly; the server never hard-codes which registry to use.
- **Pagination built in** — list/search tools accept `limit` + `offset` because component lists can be large.
- **Audit tool is static** — `get_audit_checklist` returns a fixed markdown checklist, no logic.

---

## rust-ui Implementation Plan

### Rust MCP SDK

Use the **`rmcp`** crate — the official Rust SDK from the MCP project.

```toml
# Cargo.toml
rmcp = { version = "0.1", features = ["server", "transport-io"] }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

`rmcp` provides:
- `ServerHandler` trait — implement `list_tools()` + `call_tool()`
- `stdio()` transport — wraps stdin/stdout in async framing

### Architecture

```
AI Client (Claude Code)
        |
        | stdin/stdout (JSON-RPC 2.0)
        v
   ui mcp  ←→  https://rust-ui.com/registry/
```

The server reads `ui_config.toml` for project config, fetches registry data from the public API
(same endpoints already used by `ui add`/`ui list`), and returns formatted text.

### CLI Commands

```
ui mcp                      # start stdio MCP server
ui mcp init --client X      # write editor config file
```

Clients: `claude`, `cursor`, `vscode`, `codex`, `opencode` (same set as Shadcn).

### Tools to Expose

Map directly to existing CLI logic — no new network code needed:

| Tool | Maps To | Inputs |
|---|---|---|
| `list_components` | `ui list` logic | `category?` (string) |
| `search_components` | `ui search` logic | `query` (string) |
| `view_component` | `ui view` logic | `name` (string) |
| `get_add_command` | static format | `components[]` |
| `get_component_info` | registry tree fetch | `name` (string) |
| `get_audit_checklist` | static text | none |

#### Tool descriptions (what the AI sees)

```
list_components     — List all available rust-ui components, optionally filtered by category
search_components   — Fuzzy search for components by name
view_component      — View the full source code of a component
get_add_command     — Returns the 'ui add <name>' command to install components
get_component_info  — Get metadata: dependencies, files, category for a component
get_audit_checklist — Checklist to verify after adding components (imports, Cargo.toml, etc.)
```

### File Structure

```
src/command_mcp/
    mod.rs          # re-exports
    _mcp.rs         # McpServer struct implementing ServerHandler, tool dispatch
    config.rs       # mcp init subcommand: writes editor config files
    tools.rs        # one fn per tool, returns String (formatted text for AI)
    formatter.rs    # format component lists, search results, source code
```

### Server Entry Point (sketch)

```rust
// src/command_mcp/_mcp.rs

use rmcp::{ServerHandler, model::*, service::RequestContext, tool};

pub struct RustUiMcpServer;

#[tool(tool_box)]
impl RustUiMcpServer {
    #[tool(description = "List all available rust-ui components")]
    async fn list_components(
        &self,
        #[tool(param)] category: Option<String>,
    ) -> String {
        tools::list_components(category).await
            .unwrap_or_else(|e| format!("Error: {e}"))
    }

    #[tool(description = "Search for rust-ui components by name")]
    async fn search_components(
        &self,
        #[tool(param)] query: String,
    ) -> String {
        tools::search_components(&query).await
            .unwrap_or_else(|e| format!("Error: {e}"))
    }

    #[tool(description = "View the full source code of a component")]
    async fn view_component(
        &self,
        #[tool(param)] name: String,
    ) -> String {
        tools::view_component(&name).await
            .unwrap_or_else(|e| format!("Error: {e}"))
    }

    #[tool(description = "Get the 'ui add' command to install one or more components")]
    async fn get_add_command(
        &self,
        #[tool(param)] components: Vec<String>,
    ) -> String {
        format!("ui add {}", components.join(" "))
    }

    #[tool(description = "Get metadata for a component: files, deps, category")]
    async fn get_component_info(
        &self,
        #[tool(param)] name: String,
    ) -> String {
        tools::component_info(&name).await
            .unwrap_or_else(|e| format!("Error: {e}"))
    }

    #[tool(description = "Checklist to verify after adding components")]
    async fn get_audit_checklist(&self) -> String {
        tools::audit_checklist()
    }
}

pub async fn run_mcp_server() -> anyhow::Result<()> {
    let transport = rmcp::transport::stdio();
    let server = RustUiMcpServer;
    rmcp::serve_server(server, transport).await?;
    Ok(())
}
```

### mcp init (editor config writing)

```rust
// src/command_mcp/config.rs

pub enum McpClient { Claude, Cursor, VsCode, Codex, OpenCode }

pub fn write_mcp_config(client: McpClient, cwd: &Path) -> CliResult<String> {
    match client {
        McpClient::Claude => {
            // writes .mcp.json
            let config = serde_json::json!({
                "mcpServers": {
                    "rust-ui": {
                        "command": "ui",
                        "args": ["mcp"]
                    }
                }
            });
            write_json(cwd.join(".mcp.json"), config)?;
            Ok(".mcp.json".to_string())
        }
        McpClient::Cursor => {
            // writes .cursor/mcp.json
            let config = serde_json::json!({
                "mcpServers": {
                    "rust-ui": { "command": "ui", "args": ["mcp"] }
                }
            });
            write_json(cwd.join(".cursor/mcp.json"), config)?;
            Ok(".cursor/mcp.json".to_string())
        }
        McpClient::VsCode => {
            // writes .vscode/mcp.json (different key: "servers" not "mcpServers")
            let config = serde_json::json!({
                "servers": {
                    "rust-ui": { "command": "ui", "args": ["mcp"] }
                }
            });
            write_json(cwd.join(".vscode/mcp.json"), config)?;
            Ok(".vscode/mcp.json".to_string())
        }
        McpClient::Codex => {
            // cannot write ~/.codex/config.toml — print instructions instead
            Ok("(manual — see instructions above)".to_string())
        }
        McpClient::OpenCode => {
            let config = serde_json::json!({
                "$schema": "https://opencode.ai/config.json",
                "mcp": {
                    "rust-ui": {
                        "type": "local",
                        "command": ["ui", "mcp"],
                        "enabled": true
                    }
                }
            });
            write_json(cwd.join("opencode.json"), config)?;
            Ok("opencode.json".to_string())
        }
    }
}
```

### Audit Checklist (static, Rust-specific)

```
## rust-ui Audit Checklist

After adding components:

- [ ] Cargo.toml — all required crates added (leptos_ui, tw_merge, icons, etc.)
- [ ] mod.rs — component is pub mod'd correctly
- [ ] Imports — check for correct use paths (leptos::*, leptos_ui::*)
- [ ] Features — check that leptos feature flags match your project (csr/ssr/hydrate)
- [ ] Tailwind — input.css includes the component's source glob
- [ ] Browser — hot reload and check for hydration errors in console
```

---

## Integration with Main Command

```rust
// src/main.rs

.subcommand(
    Command::new("mcp")
        .about("Start the MCP server or write editor config")
        .subcommand(
            Command::new("init")
                .about("Write MCP config for your editor")
                .arg(Arg::new("client")
                    .long("client")
                    .value_parser(["claude", "cursor", "vscode", "codex", "opencode"]))
        )
)
```

When no subcommand: start the server (`run_mcp_server().await`).
When `init`: call `write_mcp_config(client, cwd)` and print the result.

---

## Differences from Shadcn

| | Shadcn (JS) | rust-ui (Rust) |
|---|---|---|
| SDK | `@modelcontextprotocol/sdk` | `rmcp` |
| Transport | `StdioServerTransport` | `rmcp::transport::stdio()` |
| Config file | `components.json` | `ui_config.toml` |
| Registry source | Pluggable, multi-registry | Single: `rust-ui.com/registry` |
| Tool naming | `search_items_in_registries` | `search_components` |
| Install command | `npx shadcn@latest add ...` | `ui add ...` |
| Examples tool | Fuzzy search for `*-demo` items | Same: search for `demo_*` components |

The rust-ui implementation can be simpler because we have a single registry, so no namespace/pagination complexity
is needed for v1. The AI workflow is the same: list → search → view → add.

---

## User-Facing Workflow (Once Implemented)

```
# 1. One-time setup
ui mcp init --client claude

# 2. Claude Code now has the rust-ui MCP server connected.
#    User can say:
"Show me all available rust-ui components"
"Add a button and a card to my project"
"What does the accordion component look like?"
"Build a login form using rust-ui components"
```

The AI calls `list_components`, `view_component`, then `get_add_command`,
and instructs the user to run `ui add button card accordion`.

---

## Implementation Order

1. Add `rmcp` dependency to `Cargo.toml`
2. Create `src/command_mcp/` with server + tools stubs
3. Wire `ui mcp` into `main.rs`
4. Implement tools (reuse existing list/search/view/registry logic)
5. Implement `ui mcp init` with config writers for all 5 clients
6. Manual test: `ui mcp` in Claude Code
7. Add to CHANGELOG + docs
