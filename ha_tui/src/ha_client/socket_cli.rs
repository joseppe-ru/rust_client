use tokio::net::UnixListener;
use tokio::net::UnixStream;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::models::SocketMessage;
use crate::models::{CliMsg, Dashboard, HaCliMsg, HaCliMsg::DATA, UserMsg};

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


        while self.running{
            tokio::select!{
                Some(msg) = self.cli_rx.recv() => {
                    println!("neue Nachricht vom CLI");
                    // self.print_debug_info("Neue Nachricht erhalten");
                    // if let Some(sock) = &mut self.sock {
                    //    // nachridcht über socket weiterleiten 
                    // println!("neue Nachricht vom CLI weiterleiten");
                    //
                    // } else {
                    //     // self.print_debug_info("Socket not connected");
                    //     println!("CLI empfang fehlgeschagen");
                    // }
                }
                Ok((mut stream, _)) = listener.accept() => {
                    println!("neuer Proband");
        
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    // tokio::spawn(async move {
                    //    self.handle_tui_connection(stream).await;
                    // });
                    stream.try_write(b"hello");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }
}
