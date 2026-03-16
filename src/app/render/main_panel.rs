use ansi_to_tui::IntoText as _;

use super::*;

impl App {
    pub fn render_main_panel(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let is_active = self.active_panel == Panel::Main;

        let block = Block::default()
            .title("Main Panel")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if is_active {
                Color::Yellow
            } else {
                Color::DarkGray
            }))
            .border_type(BorderType::Rounded);

        let raw = self.main_output_lines.lock().unwrap().join("\n");

        let text = raw.into_text().unwrap_or_else(|_| raw.as_str().into());

        let inner_height = area.height.saturating_sub(2) as usize; // subtract top+bottom borders
        let total_lines = text.lines.len();
        let max_scroll = total_lines.saturating_sub(inner_height) as u16;
        if self.output_scroll > max_scroll {
            self.output_scroll = max_scroll;
        }

        frame.render_widget(
            Paragraph::new(text)
                .block(block)
                .wrap(Wrap { trim: false })
                .scroll((self.output_scroll, 0)),
            area,
        );
    }
}
