use super::listener::HaEvent;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs::File;
// use std::io::{Read, Write};
use tokio::io::AsyncReadExt;
use std::process::Command;
use std::time::Duration;

use crate::models::{CliMsg, Dashboard, HaCliMsg, HaCliMsg::DATA, UserMsg};
use crate::ha_client::listener::HaListener;

use tokio::sync::mpsc::{Receiver, Sender};
use log::{info, warn, error, debug};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HaEntity {
    pub entity_id: String,
    state: String,
    attributes: Value,
    last_changed: String,
    #[serde(default)]
    last_reported: Option<String>,
    last_updated: String,
    pub(crate) context: Value,
}

// entity_id  state  attributes {friendly_name  supported_features}  last_changed  last_reported  last_updated  context {id parent_id user_id}
#[derive(Deserialize, Debug, Clone)]
pub struct Context {
    nods: String,
}

pub struct HAClient {
    from_tui: Receiver<UserMsg>,
    to_tui: Sender<HaCliMsg>,
    client: reqwest::Client,
    running: bool,
    start_time: std::time::Instant,
}

impl HAClient {
    pub fn new(
        from_tui: Receiver<UserMsg>, 
        to_tui: Sender<HaCliMsg>,
        start_time: std::time::Instant,
    ) -> Self {
        HAClient {
            from_tui, 
            to_tui,
            client: Self::init_client(),
            running: true,
            start_time,
        }
    }

    pub async fn run(&mut self) -> Result<(), &'static str> {
        info!("Welcome to the HA-Client TUI APP");

        // -----
        // INITIALIZING
        // -----

        // Listener für HA-Integration
        let mut ha_listener = HaListener::new(8080).await;

        // Tick für heartbeat
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));

        // self.print_debug_info("starting Main-Loop");
        while self.running {
            tokio::select! {
                Some(msg) = self.from_tui.recv() => {
                    // Updating Entity List and send it
                    //
                    // send command
                    // receiving some Data
                    self.handle_incoming(msg).await;
                }

                i = interval.tick() => {
                    self.heartbeat().await;
                }

                // Some(ha_event) = listener.recv_event() => {
                //     self.handle_incoming_ha_event(ha_event);
                // }
                //
                Some(event) = ha_listener.receiver.recv() => {
                    self.handle_ha_event(event).await;
                }

                // Ok((mut stream, _)) = self.listener.accept() => {
                //
                //     tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                //     // tokio::spawn(async move {
                //     //    self.handle_tui_connection(stream).await;
                //     // });
                //     stream.try_write(b"hello");
                //     tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                // }

                //TODO:
                // fetch updates from website on a regular basis maybe? like A Watchdog for the connection or something
                // Some(alive) = self.heartbeat() => {}
            }
        }

        Ok(())
    }


    async fn heartbeat(&mut self) -> Option<i16> {
        // self.print_debug_info("Heartbeat");
        // info!("Heartbeat");
        None
    }

    async fn handle_ha_event(&mut self, event: HaEvent) {
        // self.print_debug_info("Neues Event von HA-Integration");
        info!("HA-Event");
        // match event.event {
        //     String::from("button") => {}
        //     String::from("switch") => {}
        // }initiate_shutdown
        self.initiate_shutdown().await;
    }

    fn init_client() -> reqwest::Client {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiIyMjVkN2MwOGUwZjg0N2M5YmU5N2NlZDIwMGNiY2NmZiIsImlhdCI6MTc4MDkxMjU4MCwiZXhwIjoyMDk2MjcyNTgwfQ.gFpjPLHLu3fGk8IMC6FalquItuwTAhlDDaHDpM1DPmQ";
        let mut headers = HeaderMap::new();
        let auth_value: String = String::from("Bearer ") + token;
        headers.insert(AUTHORIZATION, auth_value.parse().unwrap());
        headers.insert(
            CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap()
    }

    async fn get_entities(&mut self)->HaCliMsg{

        let ha_url = "http://10.40.2.101:8123/api/states";
        info!("Testing connection to HA...");
        let response = self.client.get(ha_url).send().await.unwrap();

        // TODO:
        // Header auf Fehlercode analysieren

        // convert body in a Rust structured Format
        let body: Vec<HaEntity> = response.json::<Vec<HaEntity>>().await.unwrap();

        // Filter for entitys (im interested in all switches and lights)

        // body.iter().for_each(|entity| println!("{:?}", entity.context.clone()));
        info!("Fetched HA-Entitys");

        HaCliMsg::DATA {
            0: CliMsg {
                entitys: body.iter().map(|hae| hae.entity_id.clone()).collect(),
                debug_data: "Sending Entitys...".to_string(),
                dashboard: Dashboard::Dashboard_SciFi,
            },
        }

    }

    async fn handle_incoming(&mut self, msg: UserMsg) {
        info!("from tui over socket received something");
        match msg {
            UserMsg::TERMINATE => self.stop(),
            UserMsg::RESET => {
                let entities = self.get_entities().await;
                self.to_tui.try_send(entities).unwrap();},
            _ => {warn!("Komischer UserMsg-Typ");},
        }
    }

    fn send_debug_error(&mut self, msg: &str) {
        self.send_debug("ERROR", msg);
    }

    fn send_debug_info(&mut self, msg: &str) {
        self.send_debug("INFO", msg);
    }

    fn send_debug(&mut self, t: &str, s: &str) {
        match self.to_tui.try_send(HaCliMsg::DEBUG(format!(
            "{:?}-[{}]: {}",
            self.start_time.elapsed().as_millis(),t,s))) {
            Ok(_) => {
                info!("");
            }
            Err(_) => {
                warn!("MPC: communication failure, Queue full?");
            }
        };

        //senden
    }

    fn stop(&mut self) {
        error!("Stopping HA-CLI");
        self.running = false;
    }

    async fn initiate_shutdown(&mut self) {
        // self.print_debug_info("SHUTDOWN eingeleitet");
        warn!("SHUTDOWN eingeleitet");

        // let tx = self.to_tui.clone();
        tokio::spawn(async move {


            let mouse_movement = async {
                match File::open("/dev/input/mice").await {
                    Ok(mut file) => {
                        let mut buf = [0u8; 3];                         
                        // loop {
                        // if file.read_exact(&mut buf).await.is_ok() {
                        //     return; // Maus wurde bewegt
                        // }
                        // }
                       let _ = file.read_exact(&mut buf).await;
                    }
                    Err(e) => {
                        // tx.try_send(HaCliMsg::DEBUG("Berechtigunsproblem".into()));
                        // self.print_debug_error("Berechtigungs Probleme");gugu
                        error!("Berechtigungsproblem beim Shutdown");
                        std::future::pending::<()>().await;
                    }
                }
            };
        let timeout = tokio::time::sleep(Duration::from_secs(10));
        tokio::pin!(timeout);

            tokio::select! {
                _ = timeout => {
                    // tx.try_send(HaCliMsg::DEBUG("SHUTDOWN NOW".into()));
                    info!("Shutdown Now");
                    
                    let result = Command::new("systemctl")
                        .arg("poweroff")
                        .spawn();
                    if let Err(e) = result {
                        error!("Konnte Shutdown nicht ausführen: {}", e);
                    }
                }
                
                _ = mouse_movement => {
                    // tx.try_send(HaCliMsg::DEBUG("SHUTDOWN abgebrochen".into()));
                    info!("Shutdown Abgebrochen");
                }
            }
        });
    }
}


