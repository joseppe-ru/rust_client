use color_eyre::eyre;

pub mod ha_cli;
pub mod listener;
pub(crate) use ha_cli::HAClient;

// helper Function to start HA-Client
pub async fn run_ha_cli(mut app: HAClient) -> eyre::Result<(), &'static str> {
    //println!("starting ha cli");
    app.run()
        .await
        .expect("ERROR: starting Homeassistant Client");
    Ok(())
}
