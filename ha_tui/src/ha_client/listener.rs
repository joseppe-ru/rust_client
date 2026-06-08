use axum::{Json, Router, extract::State, routing::post};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::sync::mpsc;

// 1. Die Struktur der Daten, die HA uns per POST sendet
#[derive(Debug, Deserialize, Clone)]
pub struct HaEvent {
    pub event: String,
    pub entity_id: String,
}

// 2. Deine Listener-Struktur
pub struct HaListener {
    // Der Receiver, aus dem die Main-Loop die Events fischt
    pub receiver: mpsc::Receiver<HaEvent>,
}

impl HaListener {
    /// Konstruktor: Initialisiert den Server und gibt die Struktur zurück
    pub async fn new(port: u16) -> Self {
        // Erstelle den Channel für die Thread-übergreifende Kommunikation
        // 32 ist die Puffergröße (wie viele Events gecached werden, bevor der Sender blockiert)
        let (tx, rx) = mpsc::channel::<HaEvent>(32);

        // Axum Router aufsetzen und den Sender `tx` als State übergeben
        let app = Router::new()
            .route("/ha-event", post(handle_webhook))
            .with_state(tx);

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let tcp_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        // Den Server in einem neuen Task spawnen (läuft non-blocking im Hintergrund)
        tokio::spawn(async move {
            axum::serve(tcp_listener, app).await.unwrap();
        });

        // Die Struktur mit dem Receiver zurückgeben
        Self { receiver: rx }
    }
}

// 3. Der Axum-Handler (wird bei jedem HTTP-POST von HA aufgerufen)
async fn handle_webhook(
    // Extrahiert den Sender, den wir in `new` per `.with_state()` übergeben haben
    State(tx): State<mpsc::Sender<HaEvent>>,
    // Parst das ankommende JSON direkt in unser HaEvent struct
    Json(payload): Json<HaEvent>,
) -> &'static str {
    // Sende das Event in den Channel zur Main-Loop
    if let Err(e) = tx.send(payload).await {
        eprintln!("Fehler beim Senden des Events an die Main-Loop: {}", e);
    }

    // Antworte Home Assistant mit 200 OK
    "OK"
}
