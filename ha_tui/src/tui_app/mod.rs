use color_eyre::eyre;
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::models::SocketMessage;
use tokio::sync::mpsc;


use crate::models::{CliMsg, Dashboard, HaCliMsg, HaCliMsg::DATA, UserMsg};

pub mod tui_app;
pub mod intercom;
pub use self::tui_app::TuiApp;
pub use self::intercom::InterCom;

pub async fn run_tui() -> eyre::Result<(), &'static str> {
    println!("warten auf socket...");

    let time_start = std::time::Instant::now();

    let (tui_com_tx, tui_com_rx) = mpsc::channel::<UserMsg>(10);
    let (com_tui_tx, com_tui_rx) = mpsc::channel::<HaCliMsg>(10);

    let mut com = InterCom::new(tui_com_rx,com_tui_tx,time_start.clone());
    let mut app = TuiApp::new(com_tui_rx,tui_com_tx,time_start);

    println!("starting tui app");
    

    tokio::select! {
        _ = app.run() => {
            println!("App hat sich beendet. InterCom wird automatisch abgebrochen.");
        }
        
        _ = com.run() => {
            println!("InterCom hat sich beendet. App wird automatisch abgebrochen.");
        }
    }

    println!("System fährt herunter.");

    Ok(())
}
