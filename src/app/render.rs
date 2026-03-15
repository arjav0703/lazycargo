use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListDirection, ListItem, Paragraph},
};

use super::*;

impl App {
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let size = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(size);

        self.render_sidebar(frame, chunks[0]);
        // if self.popup != PopupMode::None {
        //     self.render_popup(frame);
        // }
    }

    fn render_sidebar(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let num_commands = CargoCommand::all().len() as u16;
        let num_deps = self.dependencies.len() as u16;

        let sidebar_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4 + 2),
                Constraint::Length(num_deps.max(1) + 2),
                Constraint::Length(num_commands + 2),
            ])
            .split(area);

        self.render_info_block(frame, sidebar_chunks[0]);
        self.render_dependencies_block(frame, sidebar_chunks[1]);
        self.render_commands_block(frame, sidebar_chunks[2]);
    }

    fn render_info_block(&self, frame: &mut Frame<'_>, area: Rect) {
        let is_active =
            self.active_panel == Panel::Sidebar && self.current_section() == SidebarSection::Info;

        let block = Block::default()
            .title(SidebarSection::Info.label())
            .borders(Borders::ALL)
            .border_style(if is_active {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            });

        let package = self.manifest.package.as_ref();

        let name = package
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "unknown".to_string());
        let version = package
            .map(|p| p.version().as_local().unwrap_or("?").to_string())
            .unwrap_or_else(|| "?".to_string());
        let edition = package
            .and_then(|p| {
                p.edition.as_ref().and_then(|e| match e {
                    cargo_manifest::MaybeInherited::Local(ed) => Some(format!("{:?}", ed)),
                    cargo_manifest::MaybeInherited::Inherited { .. } => {
                        Some("workspace".to_string())
                    }
                })
            })
            .unwrap_or_else(|| "2015".to_string());

        let lines = vec![
            Line::from(vec![
                Span::styled("Name:    ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(name),
            ]),
            Line::from(vec![
                Span::styled("Version: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(version),
            ]),
            Line::from(vec![
                Span::styled("Edition: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(edition),
            ]),
        ];

        let paragraph = Paragraph::new(lines).block(block);
        frame.render_widget(paragraph, area);
    }

    fn render_dependencies_block(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let is_active = self.active_panel == Panel::Sidebar
            && self.current_section() == SidebarSection::Dependencies;

        let block = Block::default()
            .title(SidebarSection::Dependencies.label())
            .borders(Borders::ALL)
            .border_style(if is_active {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            });

        let items: Vec<ListItem> = self
            .dependencies
            .iter()
            .map(|dep| {
                let label = format!("{} v{}", dep.name, dep.version);
                ListItem::new(label)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .direction(ListDirection::TopToBottom)
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(if is_active {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    })
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.dep_list_state);
    }

    fn render_commands_block(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let is_active = self.active_panel == Panel::Sidebar
            && self.current_section() == SidebarSection::Commands;

        let block = Block::default()
            .title(SidebarSection::Commands.label())
            .borders(Borders::ALL)
            .border_style(if is_active {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            });

        let items: Vec<ListItem> = CargoCommand::all()
            .into_iter()
            .map(|cmd| ListItem::new(cmd.label().to_string()))
            .collect();

        let list = List::new(items)
            .block(block)
            .direction(ListDirection::TopToBottom)
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(if is_active {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    })
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.cmd_list_state);
    }
}
