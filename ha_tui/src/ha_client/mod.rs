use color_eyre::eyre;
use tokio::net::UnixListener;

pub mod ha_cli;
pub mod listener;
pub mod socket_cli;
use self::ha_cli::HAClient;
use self::socket_cli::SocketCli;
use crate::models::SocketMessage;
use tokio::sync::mpsc;

use log::{info, warn, error, debug};

use crate::models::{CliMsg, Dashboard, HaCliMsg, HaCliMsg::DATA, UserMsg};

// helper Function to start HA-Client
pub async fn run_ha_cli() -> eyre::Result<(), &'static str> {
    let time_start = std::time::Instant::now();

    let (com_cli_tx, com_cli_rx) = mpsc::channel::<UserMsg>(10);
    let (cli_com_tx, cli_com_rx) = mpsc::channel::<HaCliMsg>(10);

    let mut com = SocketCli::new(cli_com_rx,com_cli_tx,time_start.clone());
    let mut app = HAClient::new(com_cli_rx,cli_com_tx,time_start);

    tokio::select! {
        _ = app.run() => {
            info!("App hat sich beendet. InterCom wird automatisch abgebrochen.");
        }
        
        _ = com.run() => {
            info!("InterCom hat sich beendet. App wird automatisch abgebrochen.");
        }
    }

    info!("System fährt herunter.");

    Ok(())
}
