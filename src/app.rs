use color_eyre::Result;
use crossterm::event;
use ratatui::DefaultTerminal;

use crate::verify::verfiy_rust_project;

pub struct App {
    exit: Option<Exit>,
}

struct Exit {
    code: i32,
}

impl App {
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<i32> {
        if verfiy_rust_project().is_err() {
            self.quit(1);
        }

        while self.exit.is_none() {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events().await?;
        }

        Ok(self.exit.unwrap().code)
    }

    fn quit(&mut self, error_code: i32) {
        self.exit = Some(Exit { code: error_code });
    }

    pub fn new() -> Self {
        Self { exit: None }
    }
}

impl App {
    pub fn render(&self, frame: &mut ratatui::Frame) {
        let size = frame.area();
        let block = ratatui::widgets::Block::default()
            .title("lazycargo")
            .borders(ratatui::widgets::Borders::ALL)
            .title_alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(block, size);
    }

    pub async fn handle_crossterm_events(&mut self) -> Result<()> {
        use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
        let event = event::read()?;

        match event {
            Event::Key(k) => match k.code {
                KeyCode::Char('q') => self.quit(0),
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }
}
