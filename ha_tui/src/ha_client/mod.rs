use color_eyre::eyre;
use tokio::net::UnixListener;

pub mod ha_cli;
pub mod listener;
pub(crate) use ha_cli::HAClient;

// helper Function to start HA-Client
pub async fn run_ha_cli() -> eyre::Result<(), &'static str> {

    let time_start = std::time::Instant::now();


    let socket_path = "/tmp/mein_projekt.sock";
    
    let _ = std::fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path).unwrap();
    println!("Daemon bereit auf {}", socket_path);

    let mut app = HAClient::new(listener,time_start);


    println!("starting ha app");
    
    app.run().await.expect("ERROR: starting Homeassistant Client");
    Ok(())
}
