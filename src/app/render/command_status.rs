use super::*;

impl App {
    pub fn render_command_status(&mut self, f: &mut Frame<'_>, area: Rect) {
        match self.command_status {
            CommandStatus::Idle => {}
            CommandStatus::Running => {
                let chunks = ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Horizontal)
                    .constraints(
                        [
                            ratatui::layout::Constraint::Min(0),
                            ratatui::layout::Constraint::Length(14),
                        ]
                        .as_ref(),
                    )
                    .split(area);

                let full = throbber_widgets_tui::Throbber::default()
                    .label("Running...")
                    .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
                    .throbber_style(
                        ratatui::style::Style::default()
                            .fg(ratatui::style::Color::Red)
                            .add_modifier(ratatui::style::Modifier::BOLD),
                    )
                    .use_type(throbber_widgets_tui::WhichUse::Spin);
                self.throbber_state.calc_next();
                f.render_stateful_widget(full, chunks[1], &mut self.throbber_state);
            }
            _ => {}
        }
    }
}
