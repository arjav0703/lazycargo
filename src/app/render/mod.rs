use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListDirection, ListItem, Paragraph},
};
mod help_section;
mod main_panel;
mod popup;
mod sidebar;

use super::*;

impl App {
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let size = frame.area();

        let vert_div = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Length(1)].as_ref())
            .split(size);

        self.render_help_section(frame, vert_div[1]);

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
