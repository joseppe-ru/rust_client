#[derive(Clone)]
pub enum HaCliMsg {
    DATA(CliMsg),
    DEBUG(String),
    PACEHOLDER,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Dashboard {
    Dashboard_SciFi,
    Dashboard2,
}

pub enum UserMsg {
    READY,
    RELOAD,
    TERMINATE,
    RESET,
    DEBUG,
}
