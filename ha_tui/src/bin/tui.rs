use ha_tui::tui_app::{run_tui,TuiApp};
use tokio::{sync::mpsc, task::JoinHandle};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. UnixStream zu /tmp/dein.sock
    // 2. Socket-Daten deserialisieren -> in MPSC schieben
    // 3. run_tui(tui_app) aufrufen
    println!("TUI Client gestartet...");
    

    run_tui().await;
    Ok(())
}
