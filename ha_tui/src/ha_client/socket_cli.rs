use std::io::Bytes;
use std::ops::Deref;
use postcard::from_bytes;
use tokio::io::AsyncWriteExt;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::broadcast;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::models::{HaCliMsg, SocketMessage, UserMsg};

pub struct SocketCli{
    running: bool,
    cli_rx: Receiver<HaCliMsg>, 
    cli_tx: Sender<UserMsg>,
    start_time: std::time::Instant,
}


impl SocketCli{
    pub fn new(
        cli_rx: Receiver<HaCliMsg>, 
        cli_tx: Sender<UserMsg>,
        start_time: std::time::Instant,
    ) -> Self {
        Self {
            running: true,
            cli_rx,
            cli_tx,
            start_time,
        }
    }
    pub async fn run(&mut self){

        let socket_path = "/tmp/rust_client.sock";
        let _ = std::fs::remove_file(socket_path);

        let listener = UnixListener::bind(socket_path).unwrap();

        let (tx, _rx) = broadcast::channel::<HaCliMsg>(16);

        while self.running{
            tokio::select!{
                Some(msg) = self.cli_rx.recv() => {
                    println!("neue Nachricht vom CLI");
                    // serde msg
                    // send msg
                    let _ = tx.send(msg);

                }

                Ok((mut stream, addr)) = listener.accept() => {
                    println!("neuer Proband");
                    let tx_clone = tx.clone();
                    let rx_clone = tx.subscribe();
                    let cli_tx_clone = self.cli_tx.clone();

                    tokio::spawn(async move {
                        client_handler(stream, rx_clone, cli_tx_clone).await;
                    });


                }
            }
        }
    }

}

async fn client_handler(mut stream: UnixStream, mut rx: broadcast::Receiver<HaCliMsg>, tx: Sender<UserMsg>) {
    stream.try_write(b"Handshake-1\n");


    // Stream in Read- und Write-Hälfte aufteilen
    let (mut reader, mut writer) = stream.into_split();

    let _ = writer.try_write(b"Handshake\n");

    // Task A: Broadcast-Nachrichten (CLI -> TUI) lesen und in den Socket schreiben
    let mut write_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let sock_msg = SocketMessage::HaData(msg.clone());
            // serde
            match postcard::to_allocvec(&sock_msg) {
                Ok(binary_bytes) => {
                    println!("{}", binary_bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" "));
                    match postcard::take_from_bytes::<SocketMessage>(&binary_bytes) {
                        Ok((msg, remaining_bytes)) => {
                            println!("Serialisierung erfolgreich deserialisier");

                        }
                        Err(e) => {println!("doch nicht erfolgreich: {}",e)}
                    }
                    // Bytes per Framed-Writer senden
                    if writer.write_all(&binary_bytes).await.is_err() {
                        break;
                    }
                }
                Err(e) => eprintln!("Fehler beim Serialisieren mit Postcard: {}", e),
            }
        }
    });

    // Task B: Vom Socket lesen (TUI -> CLI) und in den MPSC-Channel schieben
    let mut read_task = tokio::spawn(async move {
        let mut buffer = [0; 1024];
        loop {
            let read_ready_result = reader.readable().await;
            if read_ready_result.is_ok() {
                match reader.try_read(&mut buffer) {
                    Ok(0) => {
                        println!("Client disconnected");
                        return
                    }
                    Ok(n) => {
                        println!("Gelesen: {} bytes", n);

                        println!("{}", buffer.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" "));

                        // take_from_bytes gibt uns die Nachricht UND den Rest des Slices zurück!
                        match postcard::take_from_bytes::<SocketMessage>(&buffer) {
                            Ok((msg, remaining_bytes)) => {
                                println!("remaining Bytes: {}",remaining_bytes.len());

                                match msg {
                                    SocketMessage::HaData(data)=>{println!("Wrong SocketMessages type")},
                                    SocketMessage::UserAction(data) => {tx.try_send(data);}
                                }
                            }
                            Err(e) => {println!("{}",e)}
                        }

                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return
                    }
                }
            }

        }
    });

    // Tasks überwachen und aufräumen
    tokio::select! {
        _ = &mut read_task => write_task.abort(),
        _ = &mut write_task => read_task.abort(),
    }

    println!("TUI-Client getrennt und Handler beendet.");

}
