use std::sync::{Arc, Mutex};

use cargo_manifest::Manifest;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Direction, Layout, Rect},
    widgets::{BorderType, Clear, ListState, Wrap},
};
use throbber_widgets_tui::ThrobberState;
use tokio::sync::mpsc;
use tui_input::Input;

use crate::domain::{CargoCommand, Dependency, DependencyExtractor};
use crate::verify::verfiy_rust_project;

mod command;
mod events;
mod helper;
mod render;
use helper::*;

pub struct CommandResult {
    pub lines: Vec<String>,
    pub success: bool,
    pub reload_manifest: bool,
}

pub struct App {
    pub manifest: Manifest,
    pub dependencies: Vec<Dependency>,

    pub active_panel: Panel,
    pub sidebar_index: usize,
    pub dep_list_state: ListState,
    pub cmd_list_state: ListState,

    pub main_output_lines: Arc<Mutex<Vec<String>>>,
    pub output_scroll: u16,
    pub command_status: CommandStatus,
    pub throbber_state: ThrobberState,
    pub last_command: Option<String>,

    pub popup: PopupMode,
    pub input: Input,

    pub cmd_result_rx: mpsc::Receiver<CommandResult>,
    pub cmd_result_tx: mpsc::Sender<CommandResult>,

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

        let (cmd_result_tx, cmd_result_rx) = mpsc::channel(4);

        Self {
            manifest,
            dependencies,
            active_panel: Panel::Sidebar,
            sidebar_index: 0,
            dep_list_state,
            cmd_list_state,
            main_output_lines: Arc::new(Mutex::new(Vec::new())),
            output_scroll: 0,
            command_status: CommandStatus::Idle,
            last_command: None,
            popup: PopupMode::None,
            throbber_state: ThrobberState::default(),
            input: Input::default(),
            cmd_result_rx,
            cmd_result_tx,
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

    /// Poll the command-result channel and apply any completed result.
    fn poll_command_result(&mut self) {
        if let Ok(result) = self.cmd_result_rx.try_recv() {
            *self.main_output_lines.lock().unwrap() = result.lines;
            self.output_scroll = 0;
            self.command_status = if result.success {
                CommandStatus::Success
            } else {
                CommandStatus::Failed
            };
            if result.reload_manifest {
                if let Ok(manifest) = Manifest::from_path("Cargo.toml") {
                    self.manifest = manifest;
                }
                self.dependencies = self.manifest.get_dependencies();
                if self.dependencies.is_empty() {
                    self.dep_list_state.select(None);
                } else {
                    self.dep_list_state.select(Some(0));
                }
            }
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<i32> {
        if verfiy_rust_project().is_err() {
            ratatui::restore();
            eprintln!("No Cargo.toml found. Please run lazycargo in the root of a Rust project.");
            return Ok(1);
        }

        while self.exit.is_none() {
            // Apply any completed background command results.
            self.poll_command_result();

            terminal.draw(|frame| {
                self.render(frame);
            })?;

            // Poll for input with a short timeout so we keep redrawing
            // (animating the throbber) even when no key is pressed.
            if event::poll(std::time::Duration::from_millis(50))? {
                self.handle_events().await?;
            }
        }

        Ok(self.exit.unwrap())
    }
}
