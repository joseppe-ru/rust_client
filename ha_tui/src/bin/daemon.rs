use ha_tui::ha_client::run_ha_cli;
use tokio::net::UnixListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env_logger::init();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // log::info!("info");
    // log::error!("error");
    // log::warn!("warning");
    // log::debug!("degub");
    run_ha_cli().await;
    Ok(())
}
