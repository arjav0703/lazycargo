use color_eyre::Result;
mod app;
mod domain;
mod verify;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let app = App::default();

    let exit_code = app.run(terminal).await;

    ratatui::restore();

    std::process::exit(exit_code?);
}
