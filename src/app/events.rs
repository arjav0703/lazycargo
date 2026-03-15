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
        todo!()
    }
}

fn handle_input_key(i: &mut Input, key: KeyEvent) {
    todo!()
}
