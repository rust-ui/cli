mod app;
mod crossterm;
mod header;
mod tabs;
mod widgets;

use std::time::Duration;

use crate::shared::cli_error::{CliError, CliResult};

/// Run the ratatui TUI for adding components
pub fn run_tui(components: Vec<String>) -> CliResult<()> {
    let tick_rate = Duration::from_millis(250);
    crossterm::run(tick_rate, components).map_err(|err| CliError::Io { source: std::io::Error::other(err.to_string()) })
}
