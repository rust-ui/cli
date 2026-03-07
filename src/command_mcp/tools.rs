use crate::command_add::tree_parser::TreeParser;
use crate::command_list::_list::{filter_by_query, format_list};
use crate::command_search::_search::format_search_result;
use crate::command_view::_view::format_view_human;
use crate::shared::cli_error::CliResult;
use crate::shared::rust_ui_client::RustUIClient;

pub async fn list_components(category: Option<String>) -> CliResult<String> {
    let tree_content = RustUIClient::fetch_tree_md().await?;
    let tree_parser = TreeParser::parse_tree_md(&tree_content)?;
    let by_category = tree_parser.get_components_by_category();

    let filtered = match &category {
        Some(cat) => filter_by_query(&by_category, cat),
        None => by_category,
    };

    Ok(format_list(&filtered))
}

pub async fn search_components(query: &str) -> CliResult<String> {
    let tree_content = RustUIClient::fetch_tree_md().await?;
    let tree_parser = TreeParser::parse_tree_md(&tree_content)?;
    let by_category = tree_parser.get_components_by_category();
    let filtered = filter_by_query(&by_category, query);
    Ok(format_search_result(&filtered, query))
}

pub async fn view_component(name: &str) -> CliResult<String> {
    let content = RustUIClient::fetch_styles_default(name).await?;
    Ok(format_view_human(name, &content))
}

pub fn audit_checklist() -> String {
    r#"## rust-ui Audit Checklist

After adding components:

- [ ] Cargo.toml — all required crates added (leptos_ui, tw_merge, icons, etc.)
- [ ] mod.rs — component is pub mod'd correctly
- [ ] Imports — correct use paths (leptos::*, leptos_ui::*)
- [ ] Features — leptos feature flags match your project (csr/ssr/hydrate)
- [ ] Tailwind — input.css includes the component's source glob
- [ ] Browser — hot reload and check for hydration errors in console"#
        .to_string()
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_checklist_contains_cargo_toml_step() {
        let checklist = audit_checklist();
        assert!(checklist.contains("Cargo.toml"));
    }

    #[test]
    fn audit_checklist_contains_all_steps() {
        let checklist = audit_checklist();
        assert!(checklist.contains("mod.rs"));
        assert!(checklist.contains("Imports"));
        assert!(checklist.contains("Features"));
        assert!(checklist.contains("Tailwind"));
        assert!(checklist.contains("Browser"));
    }

    #[test]
    fn audit_checklist_uses_markdown_checkboxes() {
        let checklist = audit_checklist();
        assert!(checklist.contains("- [ ]"));
    }
}
