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

pub fn draw_tab_demos(frame: &mut Frame, app: &mut App, area: Rect) {
    // Horizontal split: sidenav on left, detail on right
    let horizontal_chunks =
        Layout::horizontal([Constraint::Percentage(35), Constraint::Percentage(65)]).split(area);

    let (Some(&left_panel), Some(&right_panel)) = (horizontal_chunks.first(), horizontal_chunks.get(1))
    else {
        return;
    };

    // Split left panel vertically: search input at top, list below
    let left_chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(left_panel);

    let (Some(&search_area), Some(&list_area)) = (left_chunks.first(), left_chunks.get(1)) else {
        return;
    };

    // Filter demos based on search query (prefix matching)
    let demos_refs: Vec<&str> = app.demos.iter().map(|s| s.as_str()).collect();
    let filtered_demos = filter_items(&demos_refs, &app.demos_search_query);

    // Ensure scroll doesn't exceed filtered list bounds
    if !filtered_demos.is_empty() && app.demos_scroll >= filtered_demos.len() {
        app.demos_scroll = filtered_demos.len().saturating_sub(1);
    }

    // Update scrollbar state with filtered content length
    app.demos_scroll_state = app.demos_scroll_state.content_length(filtered_demos.len());

    // Left side: Demo list
    let items: Vec<ListItem> = filtered_demos
        .iter()
        .map(|demo| {
            let is_checked = app.demos_checked.contains(*demo);
            let (checkbox, color) = if is_checked { ("☑", Color::Green) } else { ("☐", Color::DarkGray) };
            ListItem::new(Span::styled(format!("  {} {}", checkbox, demo), Style::default().fg(color)))
        })
        .collect();

    let checked_count = app.demos_checked.len();
    let title = if app.demos_search_query.is_empty() {
        if checked_count > 0 {
            format!("Demos ({}) - {} Selected", app.demos.len(), checked_count)
        } else {
            format!("Demos ({})", app.demos.len())
        }
    } else if checked_count > 0 {
        format!("Demos ({}/{}) - {} Selected", filtered_demos.len(), app.demos.len(), checked_count)
    } else {
        format!("Demos ({}/{})", filtered_demos.len(), app.demos.len())
    };

    let list = List::new(items)
        .block(Block::bordered().title(title))
        .highlight_style(Style::default().bg(Color::DarkGray));

    // Update list state
    if !filtered_demos.is_empty() {
        app.demos_list_state.select(Some(app.demos_scroll));
    }

    // Draw search input in left panel
    draw_search_input(frame, &app.demos_search_query, app.demos_search_active, search_area);

    // Render list in left panel
    frame.render_stateful_widget(list, list_area, &mut app.demos_list_state);

    // Render scrollbar in left panel
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight).begin_symbol(Some("↑")).end_symbol(Some("↓")),
        list_area,
        &mut app.demos_scroll_state,
    );

    // Right side: Detail panel
    let selected_demo = filtered_demos.get(app.demos_scroll).copied();
    draw_detail_panel(frame, selected_demo, app.demos_checked.len(), "demo", right_panel);

    // Render popup if show_popup is true and there are checked demos
    if app.show_popup && !app.demos_checked.is_empty() {
        let mut checked_list: Vec<String> = app.demos_checked.iter().cloned().collect();
        checked_list.sort();
        draw_checked_popup(frame, &checked_list, "Checked Demos", "demo", Color::Green, area, 70, 60);
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

pub fn get_selected_demo(app: &App) -> Option<String> {
    let demos_refs: Vec<&str> = app.demos.iter().map(|s| s.as_str()).collect();
    get_selected_item(&demos_refs, app.demos_scroll, &app.demos_search_query).map(|s| s.to_string())
}

pub fn get_demo_at_visual_index(app: &App, visual_index: usize) -> Option<String> {
    let demos_refs: Vec<&str> = app.demos.iter().map(|s| s.as_str()).collect();
    get_item_at_visual_index(&demos_refs, visual_index, &app.demos_search_query).map(|s| s.to_string())
}
