use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, List, ListItem, Scrollbar, ScrollbarOrientation};

use super::super::app::App;
use super::super::widgets::checked_popup::draw_checked_popup;
use super::super::widgets::detail_panel::draw_detail_panel;
use super::super::widgets::helpers::{filter_items, get_item_at_visual_index, get_selected_item};
use super::super::widgets::search_input::draw_search_input;

pub fn draw_tab_hooks(frame: &mut Frame, app: &mut App, area: Rect) {
    // Horizontal split: sidenav on left, detail on right
    let horizontal_chunks =
        Layout::horizontal([Constraint::Percentage(35), Constraint::Percentage(65)]).split(area);

    // Split left panel vertically: search input at top, list below
    let left_chunks =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(horizontal_chunks[0]);

    // Filter hooks based on search query (prefix matching)
    let filtered_hooks = filter_items(HOOKS, &app.hooks_search_query);

    // Ensure scroll doesn't exceed filtered list bounds
    if !filtered_hooks.is_empty() && app.hooks_scroll >= filtered_hooks.len() {
        app.hooks_scroll = filtered_hooks.len().saturating_sub(1);
    }

    // Update scrollbar state with filtered content length
    app.hooks_scroll_state = app.hooks_scroll_state.content_length(filtered_hooks.len());

    // Left side: Hook list
    let items: Vec<ListItem> = filtered_hooks
        .iter()
        .map(|hook| {
            let is_checked = app.hooks_checked.contains(*hook);
            let (checkbox, color) = if is_checked { ("☑", Color::Green) } else { ("☐", Color::DarkGray) };
            ListItem::new(Span::styled(format!("  {} {}", checkbox, hook), Style::default().fg(color)))
        })
        .collect();

    let checked_count = app.hooks_checked.len();
    let title = if app.hooks_search_query.is_empty() {
        if checked_count > 0 {
            format!("Hooks ({}) - {} Selected", HOOKS.len(), checked_count)
        } else {
            format!("Hooks ({})", HOOKS.len())
        }
    } else if checked_count > 0 {
        format!("Hooks ({}/{}) - {} Selected", filtered_hooks.len(), HOOKS.len(), checked_count)
    } else {
        format!("Hooks ({}/{})", filtered_hooks.len(), HOOKS.len())
    };

    let list = List::new(items)
        .block(Block::bordered().title(title))
        .highlight_style(Style::default().bg(Color::DarkGray));

    // Update list state
    if !filtered_hooks.is_empty() {
        app.hooks_list_state.select(Some(app.hooks_scroll));
    }

    // Draw search input in left panel
    draw_search_input(frame, &app.hooks_search_query, app.hooks_search_active, left_chunks[0]);

    // Render list in left panel
    frame.render_stateful_widget(list, left_chunks[1], &mut app.hooks_list_state);

    // Render scrollbar in left panel
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight).begin_symbol(Some("↑")).end_symbol(Some("↓")),
        left_chunks[1],
        &mut app.hooks_scroll_state,
    );

    // Right side: Detail panel
    let selected_hook = if !filtered_hooks.is_empty() && app.hooks_scroll < filtered_hooks.len() {
        Some(filtered_hooks[app.hooks_scroll])
    } else {
        None
    };
    draw_detail_panel(frame, selected_hook, app.hooks_checked.len(), "hook", horizontal_chunks[1]);

    // Render popup if show_popup is true and there are checked hooks
    if app.show_popup && !app.hooks_checked.is_empty() {
        let mut checked_list: Vec<String> = app.hooks_checked.iter().cloned().collect();
        checked_list.sort();
        draw_checked_popup(frame, &checked_list, "Checked Hooks", "hook", Color::Green, area, 70, 60);
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

pub fn get_selected_hook(app: &App) -> Option<&'static str> {
    get_selected_item(HOOKS, app.hooks_scroll, &app.hooks_search_query)
}

pub fn get_hook_at_visual_index(app: &App, visual_index: usize) -> Option<&'static str> {
    get_item_at_visual_index(HOOKS, visual_index, &app.hooks_search_query)
}

/* ========================================================== */
/*                       ✨ CONST ✨                         */
/* ========================================================== */

const HOOKS: &[&str] = &[
    "Use Lock Body Scroll",
    "Use Horizontal Scroll",
    "Use Media Query",
    "Use Local Storage",
    "Use Debounce",
    "Use Throttle",
    "Use Click Outside",
    "Use Intersection Observer",
];
