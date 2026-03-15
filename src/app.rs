use std::sync::{Arc, Mutex};

use cargo_manifest::Manifest;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};
use tui_input::Input;

use crate::domain::{CargoCommand, Dependency, DependencyExtractor, DependencyType};
use crate::verify::verfiy_rust_project;

mod command;
mod events;
mod helper;
mod render;
use helper::*;

pub struct App {
    pub manifest: Manifest,
    pub dependencies: Vec<Dependency>,

    pub active_panel: Panel,
    pub sidebar_index: usize,
    pub dep_list_state: ListState,
    pub cmd_list_state: ListState,

    pub output_lines: Arc<Mutex<Vec<String>>>,
    pub output_scroll: u16,
    pub command_status: CommandStatus,
    pub last_command: Option<String>,

    pub popup: PopupMode,
    pub input: Input,

    exit: Option<i32>,
}

impl Default for App {
    fn default() -> Self {
        let manifest = Manifest::from_path("Cargo.toml").unwrap_or_default();
        let dependencies = manifest.get_dependencies();

        let mut dep_list_state = ListState::default();
        if !dependencies.is_empty() {
            dep_list_state.select(Some(0));
        }

        let mut cmd_list_state = ListState::default();
        cmd_list_state.select(Some(0));

        Self {
            manifest,
            dependencies,
            active_panel: Panel::Sidebar,
            sidebar_index: 0,
            dep_list_state,
            cmd_list_state,
            output_lines: Arc::new(Mutex::new(Vec::new())),
            output_scroll: 0,
            command_status: CommandStatus::Idle,
            last_command: None,
            popup: PopupMode::None,
            input: Input::default(),
            exit: None,
        }
    }
}

impl App {
    fn current_section(&self) -> SidebarSection {
        SidebarSection::all()
            .into_iter()
            .nth(self.sidebar_index)
            .unwrap_or(SidebarSection::Info)
    }

    fn quit(&mut self, code: i32) {
        self.exit = Some(code);
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<i32> {
        if verfiy_rust_project().is_err() {
            ratatui::restore();
            eprintln!("No Cargo.toml found. Please run lazycargo in the root of a Rust project.");
            return Ok(1);
        }

        while self.exit.is_none() {
            terminal.draw(|frame| {
                self.render(frame);
            })?;
            self.handle_events().await?;
        }

        Ok(self.exit.unwrap())
    }
}
