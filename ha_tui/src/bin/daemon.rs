use ha_tui::ha_client::run_ha_cli;
use tokio::net::UnixListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Daemon");
    run_ha_cli();
    Ok(())
}
