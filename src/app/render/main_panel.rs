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

        let para = self.main_output_lines.lock().unwrap().join("\n");

        frame.render_widget(
            Paragraph::new(para).block(block).wrap(Wrap { trim: false }),
            area,
        );
    }
}
