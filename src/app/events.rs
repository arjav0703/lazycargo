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
                todo!();
                // self.handle_main_key(key).await;
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
            KeyCode::Enter => {
                todo!()
            }

            KeyCode::Char('j') => {
                self.sidebar_section_down().await;
            }

            KeyCode::Char('k') => {
                self.sidebar_section_up().await;
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
    todo!()
}
