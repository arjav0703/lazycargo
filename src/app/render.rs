use super::*;
impl App {
    pub async fn render(&mut self, frame: &mut Frame<'_>) {
        let size = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(size);

        // self.render_sidebar(frame, chunks[0]).await;
        // self.render_main(frame, chunks[1]).await;
        //
        // if self.popup_mode != PopupMode::None {
        //     self.render_popup(frame).await;
        // }
    }
}
