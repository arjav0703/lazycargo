use color_eyre::Result;
use crossterm::event;
use ratatui::DefaultTerminal;

pub struct App {
    exit: bool,
}

impl App {
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events().await?;
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.exit = true;
    }

    pub fn new() -> Self {
        Self { exit: false }
    }
}

impl App {
    pub fn render(&self, frame: &mut ratatui::Frame) {
        let size = frame.area();
        let block = ratatui::widgets::Block::default()
            .title("Hello, Ratatui!")
            .borders(ratatui::widgets::Borders::ALL);
        frame.render_widget(block, size);
    }

    pub async fn handle_crossterm_events(&mut self) -> Result<()> {
        use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
        let event = event::read()?;

        match event {
            Event::Key(k) => match k.code {
                KeyCode::Char('q') => self.quit(),
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }
}
