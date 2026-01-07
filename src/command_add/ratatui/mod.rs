mod app;
mod crossterm;
mod header;
mod tabs;
mod widgets;

use std::collections::HashSet;
use std::time::Duration;

use crate::shared::cli_error::{CliError, CliResult};

/// Run the ratatui TUI for adding components
/// Returns the selected components when user confirms
pub fn run_tui(components: Vec<String>, installed: HashSet<String>) -> CliResult<Vec<String>> {
    let tick_rate = Duration::from_millis(250);
    crossterm::run(tick_rate, components, installed)
        .map_err(|err| CliError::Io { source: std::io::Error::other(err.to_string()) })
}
