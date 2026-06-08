use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SocketMessage {
    HaData(HaCliMsg),
    UserAction(UserMsg),
}


#[derive(Clone,Serialize, Deserialize)]
pub enum HaCliMsg {
    DATA(CliMsg),
    DEBUG(String),
    PACEHOLDER,
}

#[derive(Clone,Serialize, Deserialize)]
pub struct CliMsg {
    pub entitys: Vec<String>,
    pub debug_data: String,
    pub dashboard: Dashboard,
}

impl CliMsg {
    pub fn new() -> Self {
        CliMsg {
            entitys: Vec::new(),
            debug_data: "".to_string(),
            dashboard: Dashboard::Dashboard_SciFi,
        }
    }
}

#[derive(Clone,Serialize, Deserialize)]
pub enum Dashboard {
    Dashboard_SciFi,
    Dashboard2,
}

#[derive(Clone,Serialize, Deserialize)]
pub enum UserMsg {
    READY,
    RELOAD,
    TERMINATE,
    RESET,
    DEBUG,
}
