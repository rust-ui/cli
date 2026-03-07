use std::collections::BTreeMap;

use clap::Command;

use crate::command_add::tree_parser::TreeParser;
use crate::shared::cli_error::CliResult;
use crate::shared::rust_ui_client::RustUIClient;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_list() -> Command {
    Command::new("list").about("List all available components from the registry")
}

pub async fn process_list() -> CliResult<()> {
    let tree_content = RustUIClient::fetch_tree_md().await?;
    let tree_parser = TreeParser::parse_tree_md(&tree_content)?;
    let by_category = tree_parser.get_components_by_category();

    println!("{}", format_list(&by_category));
    Ok(())
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/// Pure formatter — takes grouped component data, returns the list string.
pub fn format_list(by_category: &BTreeMap<String, Vec<String>>) -> String {
    let total: usize = by_category.values().map(|v| v.len()).sum();

    if total == 0 {
        return "No components found in registry.".to_string();
    }

    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("Available components ({total} total)"));
    lines.push(String::new());

    for (category, names) in by_category {
        lines.push(format!("  {} ({})", category, names.len()));
        for name in names {
            lines.push(format!("    {name}"));
        }
        lines.push(String::new());
    }

    // Remove trailing blank line
    if lines.last().map(|l| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }

    lines.join("\n")
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    fn make_map(entries: &[(&str, &[&str])]) -> BTreeMap<String, Vec<String>> {
        entries
            .iter()
            .map(|(cat, names)| (cat.to_string(), names.iter().map(|n| n.to_string()).collect()))
            .collect()
    }

    #[test]
    fn empty_registry_shows_message() {
        let result = format_list(&make_map(&[]));
        assert_eq!(result, "No components found in registry.");
    }

    #[test]
    fn shows_total_count() {
        let map = make_map(&[("ui", &["button", "badge"]), ("demos", &["demo_button"])]);
        let result = format_list(&map);
        assert!(result.contains("3 total"));
    }

    #[test]
    fn shows_category_with_count() {
        let map = make_map(&[("ui", &["button", "badge"])]);
        let result = format_list(&map);
        assert!(result.contains("ui (2)"));
    }

    #[test]
    fn shows_each_component_on_its_own_line() {
        let map = make_map(&[("ui", &["button", "badge"])]);
        let result = format_list(&map);
        assert!(result.contains("    button"));
        assert!(result.contains("    badge"));
    }

    #[test]
    fn categories_appear_in_order() {
        let map = make_map(&[("ui", &["button"]), ("demos", &["demo_button"]), ("hooks", &["use_x"])]);
        let result = format_list(&map);
        let demos_pos = result.find("demos").unwrap();
        let hooks_pos = result.find("hooks").unwrap();
        let ui_pos = result.find("ui").unwrap();
        // BTreeMap guarantees alphabetical: demos < hooks < ui
        assert!(demos_pos < hooks_pos);
        assert!(hooks_pos < ui_pos);
    }

    #[test]
    fn single_category_single_component() {
        let map = make_map(&[("ui", &["button"])]);
        let result = format_list(&map);
        assert!(result.contains("1 total"));
        assert!(result.contains("ui (1)"));
        assert!(result.contains("    button"));
    }
}
