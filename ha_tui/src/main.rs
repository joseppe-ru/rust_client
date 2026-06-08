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
use shared_states::{CliMsg, UserMsg};
use tui_app::{run_tui, tui_app::TuiApp};
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
    //Greetings
    //    println!("Hallo Munke!");
    let time_start = std::time::Instant::now();
    // Shared Messages-System
    //let (data_tx, data_rx) = mpsc::channel::<CliMsg>(10); // For simulation data
    let (data_tx, data_rx) = mpsc::channel::<HaCliMsg>(10); // For simulation data
    let (control_tx, control_rx) = mpsc::channel::<UserMsg>(2); // For control signals

    // init apps -> load parameters ...
    let tui_app = TuiApp::new(data_rx, control_tx, time_start);
    let ha_app = HAClient::new(control_rx, data_tx, time_start);

    // start Threads for Apps with helper functions in Modules
    let tui_thread = tokio::spawn(run_tui(tui_app));
    let ha_thread = tokio::spawn(run_ha_cli(ha_app));

    // TODO: Error handling, Thread safety -> Neustarten ...
    let threadres = tokio::try_join!(flatten(tui_thread), flatten(ha_thread));
    match threadres {
        Ok((first, second)) => println!("gut 1: {:?}, gut 2: {:?}", first, second),
        Err(e) => eprintln!("Fehler: {e}"),
    }

    println!("End the world!");
    Ok(())
}
