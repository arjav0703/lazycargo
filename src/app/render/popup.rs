use super::*;

impl App {
    pub fn render_popup(&mut self, frame: &mut Frame<'_>) {
        match &self.popup {
            PopupMode::None => {}
            PopupMode::AddDependency => {
                self.render_add_dependency_popup(frame);
            }
            PopupMode::RemoveConfirm(name) => {
                let name = name.clone();
                self.render_remove_confirm_popup(frame, &name);
            }
        }
    }

    fn render_add_dependency_popup(&self, frame: &mut Frame<'_>) {
        let area = centered_rect(50, 6, frame.area());

        let block = Block::default()
            .title(" Add Dependency ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        let input_text = self.input.value();
        let inner = block.inner(area);

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);

        let input_paragraph = Paragraph::new(input_text).style(Style::default().fg(Color::White));
        frame.render_widget(input_paragraph, inner);

        // Show cursor inside the input box
        frame.set_cursor_position((inner.x + self.input.visual_cursor() as u16, inner.y));
    }

    fn render_remove_confirm_popup(&self, frame: &mut Frame<'_>, name: &str) {
        let area = centered_rect(50, 5, frame.area());

        let block = Block::default()
            .title(" Remove Dependency ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .border_type(BorderType::Rounded);

        let text = format!("Remove \"{}\"? [y/N]", name);
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        frame.render_widget(Clear, area);
        frame.render_widget(block.clone(), area);
        frame.render_widget(paragraph, block.inner(area));
    }
}

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let x = r.x + (r.width.saturating_sub(popup_width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width: popup_width.min(r.width),
        height: height.min(r.height),
    }
}
