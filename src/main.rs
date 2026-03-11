use color_eyre::Result;
mod app;

use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let mut app = App::new();

    let result = app.run(terminal).await;

    ratatui::restore();

    result
}
