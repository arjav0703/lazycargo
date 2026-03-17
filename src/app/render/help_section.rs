use super::*;

impl App {
    pub fn render_help_section(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let help_text = generate_help_text(self.current_section());

        let paragraph = Paragraph::new(help_text)
            .block(Block::default())
            .style(Style::default().fg(Color::LightMagenta));

        frame.render_widget(paragraph, area);
    }
}

fn generate_help_text(sidebar_section: SidebarSection) -> Line<'static> {
    match sidebar_section {
        SidebarSection::Info => Line::from(vec![
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": View more details  "),
        ]),

        SidebarSection::Dependencies => Line::from(vec![
            Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": Add dependency  "),
            Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": Remove dependency  "),
        ]),

        SidebarSection::Commands => Line::from(vec![
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": Execute command  "),
        ]),
    }
}
