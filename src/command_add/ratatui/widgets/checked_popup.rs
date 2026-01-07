use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Clear, Paragraph, Wrap};

use super::popup::popup_area;

/// Renders a popup showing checked items in a formatted layout
pub fn draw_checked_popup(
    frame: &mut Frame,
    checked_items: &[String],
    title: &str,
    item_type: &str,
    popup_color: Color,
    area: Rect,
    percent_x: u16,
    percent_y: u16,
) {
    let popup_block = Block::bordered().title(title).style(Style::default().fg(popup_color));
    let popup_rect = popup_area(area, percent_x, percent_y);

    // Clear the background
    frame.render_widget(Clear, popup_rect);

    let checked_text = if checked_items.is_empty() {
        format!("No {} checked", item_type)
    } else if checked_items.len() <= 8 {
        // Use simple vertical list for few items
        let items: Vec<String> = checked_items.iter().map(|name| format!("  ☑ {}", name)).collect();
        let item_type_display =
            if checked_items.len() == 1 { item_type.to_string() } else { format!("{}s", item_type) };
        format!(
            "Checked {} ({})\n\n{}\n\n\nPress ENTER to add  |  Press ESC to close",
            item_type_display,
            checked_items.len(),
            items.join("\n")
        )
    } else {
        // Format items in 4 columns for many items
        let items_per_column = checked_items.len().div_ceil(4);
        let item_type_display =
            if checked_items.len() == 1 { item_type.to_string() } else { format!("{}s", item_type) };
        let mut lines = vec![format!("Checked {} ({})\n", item_type_display, checked_items.len())];

        for row in 0..items_per_column {
            let mut line_parts = Vec::new();

            // Column 1
            if let Some(item) = checked_items.get(row) {
                line_parts.push(format!("  ☑ {:<18}", item));
            }

            // Column 2
            if let Some(item) = checked_items.get(row + items_per_column) {
                line_parts.push(format!("☑ {:<18}", item));
            }

            // Column 3
            if let Some(item) = checked_items.get(row + items_per_column * 2) {
                line_parts.push(format!("☑ {:<18}", item));
            }

            // Column 4
            if let Some(item) = checked_items.get(row + items_per_column * 3) {
                line_parts.push(format!("☑ {}", item));
            }

            lines.push(line_parts.join("  "));
        }

        lines.push(String::new());
        lines.push(String::new());
        lines.push("Press ENTER to add  |  Press ESC to close".to_string());
        lines.join("\n")
    };

    let popup_paragraph = Paragraph::new(checked_text)
        .block(popup_block)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White));

    frame.render_widget(popup_paragraph, popup_rect);
}
