use color_eyre::Result;
mod app;
mod dependencies;
mod verify;
use app::App;
use dependencies::DependencyExtractor;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let mut app = App::new();

    let exit_code = app.run(terminal).await;

    ratatui::restore();

    let dependencies = dependencies::get_manifest().get_dependencies();
    dbg!("Dependencies: {:#?}", dependencies);

    std::process::exit(exit_code?);
    // Ok(())
}
