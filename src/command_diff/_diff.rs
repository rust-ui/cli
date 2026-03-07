use std::path::Path;

use clap::{Arg, ArgMatches, Command};
use colored::Colorize;
use serde::Serialize;

use crate::command_add::component_type::ComponentType;
use crate::command_add::installed::get_installed_components;
use crate::command_init::config::UiConfig;
use crate::shared::cli_error::CliResult;
use crate::shared::rust_ui_client::RustUIClient;

const UI_CONFIG_TOML: &str = "ui_config.toml";
const CONTEXT_LINES: usize = 3;

/* ========================================================== */
/*                        📦 TYPES 📦                         */
/* ========================================================== */

#[derive(Debug, PartialEq, Clone)]
pub enum DiffLine {
    Same(String),
    Removed(String),
    Added(String),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffStatus {
    UpToDate,
    Changed,
    NotInRegistry,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffHunk {
    pub removed: Vec<String>,
    pub added: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ComponentDiff {
    pub name: String,
    pub status: DiffStatus,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Serialize)]
struct ComponentDiffJson {
    name: String,
    status: DiffStatus,
    hunks: Vec<DiffHunk>,
}

/* ========================================================== */
/*                        🔧 COMMAND 🔧                       */
/* ========================================================== */

pub fn command_diff() -> Command {
    Command::new("diff")
        .about("Show line-by-line diff of installed components vs the registry")
        .arg(Arg::new("component").help("Component name to diff (omit to diff all installed)").required(false))
        .arg(Arg::new("json").long("json").help("Output as JSON").action(clap::ArgAction::SetTrue))
}

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

/// Fetch registry content and compute diffs for a list of component names.
/// Names are processed in the order given; sort before calling if needed.
pub async fn diff_components(names: &[String], base_path: &str) -> CliResult<Vec<ComponentDiff>> {
    let mut diffs: Vec<ComponentDiff> = Vec::new();
    for name in names {
        let component_type = ComponentType::from_component_name(name);
        let local_path = Path::new(base_path).join(component_type.to_path()).join(format!("{name}.rs"));
        match RustUIClient::fetch_styles_default(name).await {
            Ok(remote_content) => {
                let local_content = std::fs::read_to_string(&local_path).unwrap_or_default();
                let diff_lines = compute_diff(&local_content, &remote_content);
                let has_changes = diff_lines.iter().any(|l| !matches!(l, DiffLine::Same(_)));
                let status = if has_changes { DiffStatus::Changed } else { DiffStatus::UpToDate };
                diffs.push(ComponentDiff { name: name.clone(), status, lines: diff_lines });
            }
            Err(_) => {
                diffs.push(ComponentDiff { name: name.clone(), status: DiffStatus::NotInRegistry, lines: vec![] });
            }
        }
    }
    Ok(diffs)
}

pub async fn process_diff(matches: &ArgMatches) -> CliResult<()> {
    let json = matches.get_flag("json");
    let component_arg: Option<&String> = matches.get_one("component");

    let config = UiConfig::try_reading_ui_config(UI_CONFIG_TOML)?;
    let base_path = config.base_path_components;

    let names: Vec<String> = if let Some(name) = component_arg {
        vec![name.clone()]
    } else {
        let mut installed: Vec<String> = get_installed_components(&base_path).into_iter().collect();
        installed.sort();
        installed
    };

    if names.is_empty() {
        println!("No components installed.");
        return Ok(());
    }

    if component_arg.is_none() {
        println!("Checking {} installed component{}...\n", names.len(), if names.len() == 1 { "" } else { "s" });
    }

    let diffs = diff_components(&names, &base_path).await?;

    let output = if json { format_diff_json(&diffs)? } else { format_diff_human(&diffs) };
    println!("{output}");

    Ok(())
}

/* ========================================================== */
/*                      🧮 ALGORITHM 🧮                       */
/* ========================================================== */

/// Compute a line-level diff using LCS (Longest Common Subsequence).
pub fn compute_diff(local: &str, remote: &str) -> Vec<DiffLine> {
    let local_lines: Vec<&str> = local.lines().collect();
    let remote_lines: Vec<&str> = remote.lines().collect();

    let m = local_lines.len();
    let n = remote_lines.len();

    // Build LCS table
    let mut table = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if local_lines[i - 1] == remote_lines[j - 1] {
                table[i][j] = table[i - 1][j - 1] + 1;
            } else {
                table[i][j] = table[i - 1][j].max(table[i][j - 1]);
            }
        }
    }

    // Backtrack to build diff
    let mut result: Vec<DiffLine> = Vec::new();
    let mut i = m;
    let mut j = n;

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && local_lines[i - 1] == remote_lines[j - 1] {
            result.push(DiffLine::Same(local_lines[i - 1].to_string()));
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
            result.push(DiffLine::Added(remote_lines[j - 1].to_string()));
            j -= 1;
        } else {
            result.push(DiffLine::Removed(local_lines[i - 1].to_string()));
            i -= 1;
        }
    }

    result.reverse();
    result
}

/* ========================================================== */
/*                      🖨  FORMATTERS 🖨                      */
/* ========================================================== */

/// Human-readable diff output with context lines.
pub fn format_diff_human(diffs: &[ComponentDiff]) -> String {
    let name_width = diffs.iter().map(|d| d.name.len()).max().unwrap_or(0);
    let mut output = String::new();

    // When showing multiple components, show a summary line for each
    let multi = diffs.len() > 1;

    let mut changed_count = 0;

    for diff in diffs {
        match diff.status {
            DiffStatus::UpToDate => {
                if multi {
                    let padded = format!("{:<width$}", diff.name, width = name_width);
                    output.push_str(&format!("  {} {}  up to date\n", "✅".green(), padded));
                }
            }
            DiffStatus::NotInRegistry => {
                let padded = format!("{:<width$}", diff.name, width = name_width);
                output.push_str(&format!("  {} {}  not in registry\n", "❓", padded));
            }
            DiffStatus::Changed => {
                changed_count += 1;
                let change_count = diff.lines.iter().filter(|l| !matches!(l, DiffLine::Same(_))).count();

                if multi {
                    let padded = format!("{:<width$}", diff.name, width = name_width);
                    output.push_str(&format!(
                        "  {} {}  changed  ({} line{})\n",
                        "⚠️ ".yellow(),
                        padded,
                        change_count,
                        if change_count == 1 { "" } else { "s" }
                    ));
                }

                // Show the actual diff block
                output.push_str(&format_single_diff(diff));
            }
        }
    }

    if multi {
        output.push('\n');
        if changed_count == 0 {
            output.push_str("All components are up to date.");
        } else {
            output.push_str(&format!(
                "{} component{} changed. Run `ui diff <name>` to inspect.",
                changed_count,
                if changed_count == 1 { " has" } else { "s have" }
            ));
        }
    }

    output
}

fn format_single_diff(diff: &ComponentDiff) -> String {
    let mut out = String::new();

    out.push_str(&format!("\n--- {} (local)\n", diff.name));
    out.push_str(&format!("+++ {} (registry)\n\n", diff.name));

    // Find indices of changed lines
    let changed_indices: Vec<usize> = diff
        .lines
        .iter()
        .enumerate()
        .filter(|(_, l)| !matches!(l, DiffLine::Same(_)))
        .map(|(i, _)| i)
        .collect();

    if changed_indices.is_empty() {
        return out;
    }

    // Build visible ranges: CONTEXT_LINES around each changed line, merged
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for &idx in &changed_indices {
        let start = idx.saturating_sub(CONTEXT_LINES);
        let end = (idx + CONTEXT_LINES + 1).min(diff.lines.len());
        if let Some(last) = ranges.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        ranges.push((start, end));
    }

    for (range_idx, (start, end)) in ranges.iter().enumerate() {
        if range_idx > 0 {
            out.push_str(&"  ...\n".dimmed().to_string());
        }
        for line in &diff.lines[*start..*end] {
            match line {
                DiffLine::Same(s) => out.push_str(&format!("{}\n", format!("  {s}").dimmed())),
                DiffLine::Removed(s) => out.push_str(&format!("{}\n", format!("- {s}").red())),
                DiffLine::Added(s) => out.push_str(&format!("{}\n", format!("+ {s}").green())),
            }
        }
    }

    out.push('\n');
    out
}

/// Machine-readable JSON output.
pub fn format_diff_json(diffs: &[ComponentDiff]) -> CliResult<String> {
    let json_diffs: Vec<ComponentDiffJson> = diffs
        .iter()
        .map(|d| {
            let hunks = extract_hunks(&d.lines);
            ComponentDiffJson { name: d.name.clone(), status: d.status.clone(), hunks }
        })
        .collect();

    serde_json::to_string_pretty(&json_diffs).map_err(Into::into)
}

/// Extract hunks (contiguous blocks of changes) from a diff.
fn extract_hunks(lines: &[DiffLine]) -> Vec<DiffHunk> {
    let mut hunks: Vec<DiffHunk> = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if matches!(&lines[i], DiffLine::Same(_)) {
            i += 1;
            continue;
        }
        let mut removed = Vec::new();
        let mut added = Vec::new();
        while i < lines.len() && !matches!(&lines[i], DiffLine::Same(_)) {
            match &lines[i] {
                DiffLine::Removed(s) => removed.push(s.clone()),
                DiffLine::Added(s) => added.push(s.clone()),
                DiffLine::Same(_) => unreachable!(),
            }
            i += 1;
        }
        hunks.push(DiffHunk { removed, added });
    }

    hunks
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    // --- compute_diff ---

    #[test]
    fn identical_input_produces_only_same_lines() {
        let content = "fn foo() {}\nfn bar() {}";
        let diff = compute_diff(content, content);
        assert!(diff.iter().all(|l| matches!(l, DiffLine::Same(_))));
        assert_eq!(diff.len(), 2);
    }

    #[test]
    fn single_changed_line_produces_remove_and_add() {
        let local = "let x = 1;";
        let remote = "let x = 2;";
        let diff = compute_diff(local, remote);
        assert!(diff.iter().any(|l| matches!(l, DiffLine::Removed(_))));
        assert!(diff.iter().any(|l| matches!(l, DiffLine::Added(_))));
    }

    #[test]
    fn added_lines_in_remote_appear_as_added() {
        let local = "line1\nline3";
        let remote = "line1\nline2\nline3";
        let diff = compute_diff(local, remote);
        let added: Vec<_> = diff.iter().filter(|l| matches!(l, DiffLine::Added(_))).collect();
        assert_eq!(added.len(), 1);
        assert!(matches!(&added[0], DiffLine::Added(s) if s == "line2"));
    }

    #[test]
    fn removed_lines_appear_as_removed() {
        let local = "line1\nline2\nline3";
        let remote = "line1\nline3";
        let diff = compute_diff(local, remote);
        let removed: Vec<_> = diff.iter().filter(|l| matches!(l, DiffLine::Removed(_))).collect();
        assert_eq!(removed.len(), 1);
        assert!(matches!(&removed[0], DiffLine::Removed(s) if s == "line2"));
    }

    #[test]
    fn empty_inputs_produce_empty_diff() {
        let diff = compute_diff("", "");
        assert!(diff.is_empty());
    }

    #[test]
    fn multi_change_diff_preserves_same_lines() {
        let local = "a\nb\nc\nd";
        let remote = "a\nB\nc\nD";
        let diff = compute_diff(local, remote);
        let same: Vec<_> = diff.iter().filter(|l| matches!(l, DiffLine::Same(_))).collect();
        assert_eq!(same.len(), 2); // "a" and "c" are unchanged
    }

    // --- format_diff_human ---

    #[test]
    fn up_to_date_single_component_shows_no_diff_block() {
        let diff = ComponentDiff {
            name: "button".to_string(),
            status: DiffStatus::UpToDate,
            lines: vec![DiffLine::Same("fn foo() {}".to_string())],
        };
        // single component: no summary line is printed for UpToDate
        let out = format_diff_human(&[diff]);
        assert!(!out.contains("---"));
        assert!(!out.contains("+++"));
    }

    #[test]
    fn changed_component_shows_diff_headers() {
        let diff = ComponentDiff {
            name: "button".to_string(),
            status: DiffStatus::Changed,
            lines: vec![
                DiffLine::Removed("let x = 1;".to_string()),
                DiffLine::Added("let x = 2;".to_string()),
            ],
        };
        let out = format_diff_human(&[diff]);
        assert!(out.contains("--- button (local)"));
        assert!(out.contains("+++ button (registry)"));
    }

    #[test]
    fn multi_up_to_date_shows_all_up_to_date_message() {
        let diffs = vec![
            ComponentDiff { name: "badge".to_string(), status: DiffStatus::UpToDate, lines: vec![] },
            ComponentDiff { name: "card".to_string(), status: DiffStatus::UpToDate, lines: vec![] },
        ];
        let out = format_diff_human(&diffs);
        assert!(out.contains("All components are up to date."));
    }

    #[test]
    fn multi_changed_shows_changed_count() {
        let diffs = vec![
            ComponentDiff {
                name: "button".to_string(),
                status: DiffStatus::Changed,
                lines: vec![DiffLine::Added("x".to_string())],
            },
            ComponentDiff { name: "badge".to_string(), status: DiffStatus::UpToDate, lines: vec![] },
        ];
        let out = format_diff_human(&diffs);
        assert!(out.contains("1 component has changed"));
    }

    #[test]
    fn not_in_registry_shows_question_mark_label() {
        let diffs = vec![ComponentDiff {
            name: "my_custom".to_string(),
            status: DiffStatus::NotInRegistry,
            lines: vec![],
        }];
        let out = format_diff_human(&diffs);
        assert!(out.contains("not in registry"));
    }

    // --- format_diff_json ---

    #[test]
    fn json_output_is_valid_array() {
        let diffs = vec![ComponentDiff {
            name: "button".to_string(),
            status: DiffStatus::UpToDate,
            lines: vec![],
        }];
        let json = format_diff_json(&diffs).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_array());
    }

    #[test]
    fn json_status_serialized_correctly() {
        let diffs = vec![
            ComponentDiff { name: "a".to_string(), status: DiffStatus::UpToDate, lines: vec![] },
            ComponentDiff { name: "b".to_string(), status: DiffStatus::Changed, lines: vec![] },
            ComponentDiff { name: "c".to_string(), status: DiffStatus::NotInRegistry, lines: vec![] },
        ];
        let json = format_diff_json(&diffs).unwrap();
        assert!(json.contains("up_to_date"));
        assert!(json.contains("changed"));
        assert!(json.contains("not_in_registry"));
    }

    #[test]
    fn json_contains_hunks_for_changed_component() {
        let diffs = vec![ComponentDiff {
            name: "button".to_string(),
            status: DiffStatus::Changed,
            lines: vec![
                DiffLine::Same("fn foo() {}".to_string()),
                DiffLine::Removed("old".to_string()),
                DiffLine::Added("new".to_string()),
            ],
        }];
        let json = format_diff_json(&diffs).unwrap();
        assert!(json.contains("hunks"));
        assert!(json.contains("old"));
        assert!(json.contains("new"));
    }

    // --- diff_components ---

    #[tokio::test]
    async fn diff_components_empty_names_returns_empty_vec() {
        let result = diff_components(&[], "any/path").await.unwrap();
        assert!(result.is_empty());
    }
}
