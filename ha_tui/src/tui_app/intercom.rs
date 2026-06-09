use crate::models::SocketMessage;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::net::UnixStream;

use crate::models::{CliMsg, Dashboard, HaCliMsg, HaCliMsg::DATA, UserMsg};

pub struct InterCom {
    running: bool,
    tui_rx: Receiver<UserMsg>, 
    tui_tx: Sender<HaCliMsg>,
    sock: Option<UnixStream>, // Nur noch ein Stream statt zwei Halves
    start_time: std::time::Instant,
}

impl InterCom {
    pub fn new(
        tui_rx: Receiver<UserMsg>, 
        tui_tx: Sender<HaCliMsg>,
        start_time: std::time::Instant,
    ) -> Self {
        Self {
            running: true,
            tui_rx,
            tui_tx,
            sock: None,
            start_time,
        }
    }

    pub async fn run(&mut self) {
        let mut buffer = [0u8; 1024];
        // Ein Ticker, der alle 1 Sekunde anschlägt, um Reconnects zu versuchen
        let mut reconnect_timer = tokio::time::interval(tokio::time::Duration::from_secs(1));

        self.print_debug_info("Intercom aktiv");

        while self.running {
            tokio::select! {
                // 1. Nachrichten von der TUI empfangen
                Some(msg) = self.tui_rx.recv() => {
                    self.print_debug_info("Neue Nachricht erhalten");
                    if let Some(sock) = &mut self.sock {
                        // Beispiel: Nachricht an Socket senden
                        // if let Err(e) = sock.write_all(b"Daten").await {
                        //     println!("Socket Schreibfehler: {:?}", e);
                        //     self.sock = None; // Bei Fehler Verbindung trennen
                        // }
                    } else {
                        self.print_debug_info("Socket not connected");
                    }
                }

                read_ready_result = async {
                    if let Some(sock) = &self.sock {
                        sock.readable().await // Wartet auf das "Readable" Event
                    } else {
                        std::future::pending().await // Blockiert hier, wenn kein Socket existiert
                    }
                } => {
                    if read_ready_result.is_ok() {
                        if let Some(sock) = &self.sock {
                            // try_read blockiert nicht, sondern wirft WouldBlock, wenn nichts da ist
                            match sock.try_read(&mut buffer) {
                                Ok(0) => {
                                    self.print_debug_info("Socket disconnected");
                                    self.sock = None;
                                }
                                Ok(n) => {
                            
                                    self.print_debug_info(&format!("Gelesen: {} bytes", n));
                                    // TODO: Gelesene Bytes parsen und an self.tui_tx senden
                                }
                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    // False positive: Das Event wurde ausgelöst, aber es gibt (noch) keine Daten.
                                    // Wir machen im loop weiter, bis readable() wieder anschlägt.
                                    continue; 
                                }
                                Err(e) => {
                                    self.print_debug_info(&format!("Socket Lesefehler: {:?}", e));
                                    self.sock = None;
                                }
                            }
                        }
                    } else {
                                    self.print_debug_info("Fehler beim Warten auf readable()");
                        println!("Fehler beim Warten auf readable()");
                        self.sock = None;
                    }
                }

                // 3. Reconnect-Logik (blockiert die anderen Branches nicht)
                _ = reconnect_timer.tick(), if self.sock.is_none() => {
                                    self.print_debug_info("Versuche zu verbinden...");
                    match UnixStream::connect("/tmp/rust_client.sock").await {
                        Ok(stream) => {
                                    self.print_debug_info("Erfolgreich verbunden!");
                            self.sock = Some(stream);
                        }
                        Err(_) => {
                                    self.print_debug_info("Verbindung fehlgeschlagen, versuche es später erneut.");
                        }
                    }
                }
            }
        }
    }

    fn print_debug_error(&mut self, msg: &str) {
        self.print_debug("ERROR", msg);
    }

    fn print_debug_info(&mut self, msg: &str) {
        self.print_debug("INFO", msg);
    }

    fn print_debug(&mut self, t: &str, s: &str) {
        match self.tui_tx.try_send(HaCliMsg::DEBUG(format!(
            "{:?}-[{}]: {}",
            self.start_time.elapsed().as_millis(),t,s))) {
            Ok(_) => {
                println!("");
            }
            Err(_) => {
                println!("MPC: communication failure, Queue full?");
            }
        };

        //senden
    }
}
