use super::*;

impl App {
    pub async fn handle_events(&mut self) -> Result<()> {
        let ev = event::read()?;

        match &self.popup {
            PopupMode::AddDependency => {
                if let Event::Key(key) = ev {
                    match key.code {
                        KeyCode::Esc => {
                            self.popup = PopupMode::None;
                            self.input = Input::default();
                        }
                        KeyCode::Enter => {
                            let crate_name = self.input.value().trim().to_string();
                            if !crate_name.is_empty() {
                                self.popup = PopupMode::None;
                                self.input = Input::default();
                                self.run_cargo_add(crate_name).await;
                            }
                        }
                        _ => {
                            handle_input_key(&mut self.input, key);
                        }
                    }
                }
                return Ok(());
            }
            PopupMode::RemoveConfirm(name) => {
                let name = name.clone();
                if let Event::Key(key) = ev {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                            self.popup = PopupMode::None;
                        }
                        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                            self.popup = PopupMode::None;
                            self.run_cargo_remove(name).await;
                        }
                        _ => {}
                    }
                }
                return Ok(());
            }
            PopupMode::None => {}
        }

        if let Event::Key(key) = ev {
            self.handle_key(key).await;
        }

        Ok(())
    }

    async fn handle_key(&mut self, key: KeyEvent) {
        if key == KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE) {
            self.quit(0);
            return;
        }
        match self.active_panel {
            Panel::Sidebar => {
                self.handle_sidebar_key(key).await;
            }
            Panel::Main => {
                self.handle_main_key(key).await;
            }
        }
    }

    async fn handle_sidebar_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('1') => {
                self.sidebar_index = 0;
            }
            KeyCode::Char('2') => {
                self.sidebar_index = 1;
            }
            KeyCode::Char('3') => {
                self.sidebar_index = 2;
            }
            KeyCode::Enter => match self.current_section() {
                SidebarSection::Info => {
                    todo!()
                }
                SidebarSection::Commands => {
                    if let Some(i) = self.cmd_list_state.selected() {
                        let cmd = CargoCommand::all()[i].clone();
                        self.run_cargo_command(cmd).await;
                    }
                }
                _ => {}
            },
            KeyCode::Tab => {
                self.active_panel = Panel::Main;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.sidebar_section_down().await;
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.sidebar_section_up().await;
            }
            _ => {}
        }

        if self.current_section() == SidebarSection::Dependencies {
            match key.code {
                KeyCode::Char('a') => {
                    self.popup = PopupMode::AddDependency;
                }
                KeyCode::Backspace | KeyCode::Delete | KeyCode::Char('d') => {
                    if let Some(i) = self.dep_list_state.selected()
                        && let Some(dep) = self.dependencies.get(i)
                    {
                        self.popup = PopupMode::RemoveConfirm(dep.name.clone());
                    }
                }
                _ => {}
            }
        }
    }

    async fn handle_main_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab | KeyCode::Esc => {
                self.active_panel = Panel::Sidebar;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.output_scroll = self.output_scroll.saturating_add(1);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.output_scroll = self.output_scroll.saturating_sub(1);
            }
            KeyCode::Char('d') | KeyCode::PageDown => {
                self.output_scroll = self.output_scroll.saturating_add(10);
            }
            KeyCode::Char('u') | KeyCode::PageUp => {
                self.output_scroll = self.output_scroll.saturating_sub(10);
            }
            KeyCode::Char('g') | KeyCode::Home => {
                self.output_scroll = 0;
            }
            KeyCode::Char('G') | KeyCode::End => {
                self.output_scroll = u16::MAX;
            }
            _ => {}
        }
    }

    async fn sidebar_section_down(&mut self) {
        let section = self.current_section();
        match section {
            SidebarSection::Info => {}
            SidebarSection::Dependencies => {
                if !self.dependencies.is_empty() {
                    let i = self.dep_list_state.selected().unwrap_or(0);
                    let next_i = (i + 1).min(self.dependencies.len() - 1);
                    self.dep_list_state.select(Some(next_i));
                }
            }
            SidebarSection::Commands => {
                let i = self.cmd_list_state.selected().unwrap_or(0);
                let next_i = (i + 1).min(CargoCommand::all().len() - 1);
                self.cmd_list_state.select(Some(next_i));
            }
        }
    }

    async fn sidebar_section_up(&mut self) {
        let section = self.current_section();
        match section {
            SidebarSection::Info => {}
            SidebarSection::Dependencies => {
                if !self.dependencies.is_empty() {
                    let i = self.dep_list_state.selected().unwrap_or(0);
                    let next_i = i.saturating_sub(1);
                    self.dep_list_state.select(Some(next_i));
                }
            }
            SidebarSection::Commands => {
                let i = self.cmd_list_state.selected().unwrap_or(0);
                let next_i = i.saturating_sub(1);
                self.cmd_list_state.select(Some(next_i));
            }
        }
    }
}

fn handle_input_key(i: &mut Input, key: KeyEvent) {
    use tui_input::InputRequest;
    let req = match (key.code, key.modifiers) {
        (KeyCode::Backspace, KeyModifiers::NONE) => Some(InputRequest::DeletePrevChar),
        (KeyCode::Delete, KeyModifiers::NONE) => Some(InputRequest::DeleteNextChar),
        (KeyCode::Left, KeyModifiers::NONE) => Some(InputRequest::GoToPrevChar),
        (KeyCode::Right, KeyModifiers::NONE) => Some(InputRequest::GoToNextChar),
        (KeyCode::Home, KeyModifiers::NONE) => Some(InputRequest::GoToStart),
        (KeyCode::End, KeyModifiers::NONE) => Some(InputRequest::GoToEnd),
        (KeyCode::Char(c), KeyModifiers::NONE) => Some(InputRequest::InsertChar(c)),
        (KeyCode::Char(c), KeyModifiers::SHIFT) => Some(InputRequest::InsertChar(c)),
        _ => None,
    };
    if let Some(req) = req {
        i.handle(req);
    }
}
