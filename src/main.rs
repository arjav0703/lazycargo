use color_eyre::Result;
mod app;
mod domain;
mod verify;
use app::App;
use domain::*;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let mut app = App::new();

    let dependencies = app.manifest.clone().get_dependencies();

    let exit_code = app.run(terminal).await;

    ratatui::restore();

    dbg!("Dependencies: {:#?}", dependencies);

    std::process::exit(exit_code?);
    // Ok(())
}
