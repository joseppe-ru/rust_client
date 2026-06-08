use color_eyre::eyre;

pub mod tui_app;
use tui_app::TuiApp;

// helper Function to start TUI
pub async fn run_tui(mut app: TuiApp) -> eyre::Result<(), &'static str> {
    println!("starting tui app");
    app.run().await.expect("ERROR: starting Terminal UI");
    Ok(())
}
