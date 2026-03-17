use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListDirection, ListItem, Paragraph},
};
mod command_status;
mod help_section;
mod main_panel;
mod popup;
mod sidebar;

use super::*;

impl App {
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();

        // total area -> main area + footer
        let vert_div = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Length(1)].as_ref())
            .split(area);

        // footer -> help section on the left, command status overlays the full footer row
        let footer = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(vert_div[1]);

        self.render_help_section(frame, footer[0]);
        self.render_command_status(frame, vert_div[1]);

        // main area -> sidebar + main panel
        let main_div = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(vert_div[0]);

        self.render_sidebar(frame, main_div[0]);
        self.render_main_panel(frame, main_div[1]);
        if self.popup != PopupMode::None {
            self.render_popup(frame);
        }
    }
}
