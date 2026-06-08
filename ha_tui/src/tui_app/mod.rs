use color_eyre::eyre;
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


pub mod tui_app;
pub use self::tui_app::TuiApp;

pub async fn run_tui() -> eyre::Result<(), &'static str> {
    println!("warten auf socket...");

    let time_start = std::time::Instant::now();
    let stream = UnixStream::connect("/tmp/mein_projekt.sock").await.unwrap();
    let (mut reader, mut writer) = stream.into_split();

    let mut app = TuiApp::new(writer,reader,time_start);

    println!("starting tui app");
    
    app.run().await.expect("ERROR: starting Terminal UI");
    Ok(())
}
