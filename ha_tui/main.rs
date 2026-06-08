// used rust crates
use color_eyre::eyre;
use tokio::{sync::mpsc, task::JoinHandle};

// own Modules
mod ha_client;
mod shared_states;
mod tui_app;

// usage of own Struct
use crate::shared_states::HaCliMsg;
use ha_client::{HAClient, run_ha_cli};
use shared_states::{CliMsg, UserMsg, SocketMessage};
//use tui_app::{run_tui, tui_app::TuiApp};
// TODO: Bennenung der Apps ändern

async fn flatten<T>(handle: JoinHandle<eyre::Result<T, &'static str>>) -> Result<T, &'static str> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err),
        Err(_err) => Err("handling failed"),
    }
}

//TODO: bei crach neu starten

// Einstiegspunkt
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time_start = std::time::Instant::now();
    // daemon
#[cfg(feature = "daemon")]
    {
        run_daemon().await?
    }
    
#[cfg(feature = "tui")]
    {
        run_tui().await?
    }
    Ok(())
}

async fn run_daemon() -> Result<(), Box<dyn std::error::Error>>{
    //
    // let listener = UnixListener::bind("/tmp/my_app.sock")?;
    //
    // let (data_tx, data_rx) = mpsc::channel::<HaCliMsg>(10); 
    // let (control_tx, control_rx) = mpsc::channel::<UserMsg>(2); 
    //
    // // init apps -> load parameters ...
    // // let tui_app = TuiApp::new(data_rx, control_tx, time_start);
    // let ha_app = HAClient::new(control_rx, data_tx, time_start);
    //
    // // start Threads for Apps with helper functions in Modules
    // // let tui_thread = tokio::spawn(run_tui(tui_app));
    // tokio::spawn(run_ha_cli(ha_app));
    //
    // loop {
    //     let (mut socket, _) = listener.accept().await?;
    //
    //     // Bei jedem neuen TUI-Client spawnen wir einen Task, 
    //     // der den aktuellen State sendet
    //     tokio::spawn(async move {
    //         // Hier würdest du die Nachricht aus dem `rx` (oder einem Broadcast-Kanal) 
    //         // serialisieren und über den Socket schicken
    //         let msg = AppMessage::HaEvent("Update".to_string());
    //         let encoded = bincode::serialize(&msg).unwrap();
    //         socket.write_all(&encoded).await.unwrap();
    //     });
    // }
    //
   println!("run_daemon"); 
    Ok(())
}

async fn run_tui()-> Result<(), Box<dyn std::error::Error>>{
    println!("run_tui");
    Ok(())
}

