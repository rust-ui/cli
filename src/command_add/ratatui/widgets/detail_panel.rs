use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph};

/// Renders a detail panel showing the selected item and instructions
pub fn draw_detail_panel(
    frame: &mut Frame,
    selected_item: Option<&str>,
    checked_count: usize,
    item_type: &str,
    area: Rect,
) {
    let content_block = Block::bordered().title("Detail");
    let inner_area = content_block.inner(area);
    frame.render_widget(content_block, area);

    if let Some(item) = selected_item {
        let instruction = if checked_count > 0 {
            let item_type_display = if checked_count == 1 { item_type.to_string() } else { format!("{}s", item_type) };
            format!(
                "\n\n\n({} {} checked)\nPress ENTER to view checked {}",
                checked_count, item_type_display, item_type_display
            )
        } else {
            String::new()
        };

        let text = format!("Selected: {}\n\n{} details will be displayed here...{}", item, item_type, instruction);
        let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White));
        frame.render_widget(paragraph, inner_area);
    } else {
        let paragraph = Paragraph::new(format!("Select a {}", item_type)).style(Style::default().fg(Color::Gray));
        frame.render_widget(paragraph, inner_area);
    }
}
