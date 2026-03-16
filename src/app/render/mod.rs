use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListDirection, ListItem, Paragraph},
};
mod main_panel;
mod popup;
mod sidebar;

use super::*;

impl App {
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let size = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(size);

        self.render_sidebar(frame, chunks[0]);
        self.render_main_panel(frame, chunks[1]);
        if self.popup != PopupMode::None {
            self.render_popup(frame);
        }
    }
}
