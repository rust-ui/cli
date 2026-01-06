use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Clear, Paragraph, Wrap};

use super::super::app::App;
use super::super::header::Tab;
use super::super::widgets::popup::popup_area;
use super::{tab1_components, tab2_hooks, tab3_blocks, tab4_icons, tab9_settings};

pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

    // Render header with tabs
    app.header.render(frame, chunks[0]);

    match app.header.tabs.current {
        Tab::Components => tab1_components::draw_tab_components(frame, app, chunks[1]),
        Tab::Hooks => tab2_hooks::draw_tab_hooks(frame, app, chunks[1]),
        Tab::Blocks => tab3_blocks::draw_tab_blocks(frame, app, chunks[1]),
        Tab::Icons => tab4_icons::draw_tab_icons(frame, app, chunks[1]),
        Tab::Settings => tab9_settings::draw_tab_settings(frame, app, chunks[1]),
    };

    // Render help popup on top of everything
    if app.show_help_popup {
        draw_help_popup(frame, frame.area());
    }
}

fn draw_help_popup(frame: &mut Frame, area: Rect) {
    let popup_block = Block::bordered().title("⌨️  Keyboard Shortcuts").style(Style::default().fg(Color::Cyan));
    let popup_area = popup_area(area, 75, 85);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    let help_text = r#"
Global Commands:
  ?            Show this help menu
  q            Quit the application
  h / ←        Navigate to previous tab
  l / →        Navigate to next tab
  j / ↓        Scroll down
  k / ↑        Scroll up

Components Tab:
  /            Activate search mode
  Space        Toggle checkbox for selected component
  Enter        View checked components (when components are selected)
  Esc Esc      Double-tap Escape to deselect all components
  Double-click Toggle checkbox on clicked component
  Mouse Wheel  Scroll up/down

Search Mode (Components Tab):
  j / ↓        Navigate down while searching
  k / ↑        Navigate up while searching
  Esc          Exit search mode
  Backspace    Delete character
  Any char     Add to search query


Press ESC to close this help menu
"#;

    let popup_paragraph =
        Paragraph::new(help_text).block(popup_block).wrap(Wrap { trim: true }).style(Style::default().fg(Color::White));

    frame.render_widget(popup_paragraph, popup_area);
}
