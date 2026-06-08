use tokio::sync::mpsc::{Receiver, Sender};
use tokio::net::unix::{OwnedWriteHalf,OwnedReadHalf};
use tokio::io::AsyncReadExt;
// use ha_tui::models::HaCliMsg::DATA;
use crate::models::{CliMsg, Dashboard, HaCliMsg, HaCliMsg::DATA, UserMsg};
use crossterm::event::{self, Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use ratatui::Frame;
use ratatui::layout::{
    Alignment, Constraint,
    Constraint::{Max, Min},
    Layout, Margin, Offset, Rect,
};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::canvas::{Canvas, Circle, Context};
use ratatui::widgets::{
    Block, BorderType, Borders, List, ListDirection, ListItem, ListState, Paragraph, Widget, Wrap,
};

// Konstanten bleiben gleich
const RECT_COUNTS: [usize; 7] = [3, 1, 1, 0, 1, 2, 2];
const RECT_COLOR: Color = Color::DarkGray;
const RECT_WIDTH: u16 = 5; // Etwas breiter, damit Text besser passt
const RECT_HEIGHT: u16 = 2;

const Max_DBG_MSGS: u16 = 20;

// TODO: Use for text input, not ready yet
enum InputModeE {
    NORMAL,
    EDIT_DURATION,
    EDIT_SPEEDUP,
}
// Klasse der TUI-APP mit Membervariablen
pub struct TuiApp {
    running: bool,
    writer: OwnedWriteHalf,
    reader: OwnedReadHalf,
    input_mode: InputModeE,
    entities: Vec<String>,
    state: ListState,
    event_stream: EventStream,
    dbmsg_buf: Vec<String>,
    start_time: std::time::Instant,
}

// TODO: Dashboards für bestimmte Zwecke
// TODO: Pop-Ups

impl TuiApp {
    pub fn new(
    writer: OwnedWriteHalf,
    reader: OwnedReadHalf,
        start_time: std::time::Instant,
    ) -> Self {
        // Return a new instance of TuiApp
        Self {
            running: false,
            writer,
            reader,
            input_mode: InputModeE::NORMAL,
            entities: Vec::new(),
            state: ListState::default(),
            event_stream: EventStream::new(),
            dbmsg_buf: Vec::new(),
            start_time,
        }
    }

    // Main Loop for TUI
    pub async fn run(&mut self) -> Result<(), &'static str> {
        self.running = true;
        self.state.select(Some(0));
        // Initialize the terminal UI
        let mut terminal = ratatui::init();
        // match self.to_ha.send(UserMsg::READY).await {
        //     Ok(_)=>(),
        //     Err(e)=> eprintln!("{}", e),
        // }

        let mut data: HaCliMsg = HaCliMsg::PACEHOLDER;
let mut buffer = [0; 1024];

        while self.running {
            tokio::select! {
                Some(Ok(event)) = self.event_stream.next() => {
                    self.handle_events(event).await;
                }
                
                result = self.reader.read(&mut buffer) => {
                    data=HaCliMsg::DEBUG(format!("hello"));
                }
                //Nachrichten empfangen
                // Some(msg) = self.from_ha.recv() => {
                //     data = msg;
                // }
            }
            terminal
                .draw(|f| self.render(f, data.clone()))
                .expect("TODO: panic cuhstring message");
            data = HaCliMsg::PACEHOLDER;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, data: HaCliMsg) {
        self.render_db_scifi(frame, data);
        /*
        match data.dashboard {
            Dashboard::Dashboard_SciFi => {self.render_db_scifi(frame, data)}
            _ => {}
        }
         */
    }

    fn render_db_scifi(&mut self, frame: &mut Frame, data: HaCliMsg) {
        //TODO: Daten aus der Message vom Client werden an diesem Punkt aktualisiert
        // je nach dem, was ankommt, sonst bleibt das hoffentlich gleich
        match data {
            HaCliMsg::DATA(d) => {
                self.entities = d.entitys.clone();
            }
            HaCliMsg::DEBUG(d) => {
                self.dbmsg_buf.insert(0, d.clone());
                if self.dbmsg_buf.len() > Max_DBG_MSGS as usize {
                    self.dbmsg_buf.truncate(Max_DBG_MSGS as usize);
                }
            }
            HaCliMsg::PACEHOLDER => {}
        }

        // Bildschirm in drei Bereiche einteilen (unterainander)
        let vertical = Layout::vertical([Max(8), Min(9), Max(4)]);
        let [area_graphs, area_control, area_stats] = vertical.areas(frame.area());

        // Graph-Bereich (oben) in 4 Abschnitte unterteilen
        let graphs_horizontal = Layout::horizontal([Min(9), Min(9), Min(9), Min(9)]);
        let [
            area_humid_graph,
            area_temp_graph,
            area_power_graph,
            area_fun_graph,
        ] = graphs_horizontal.areas(area_graphs);

        // Control-Ebene in 3 Breiche aufteilen (auswählbare entitäten, Wohnung Grundriss, Debug bzw. Beschreibungs-textfeld)
        let control_horizontal = Layout::horizontal([Max(20), Min(8), Max(30)]);
        let home_vertical = Layout::vertical([Min(5), Min(5), Min(5)]);
        let [area_sel_list, area_home, area_debug] = control_horizontal.areas(area_control);
        let [area_top, are_mid, area_bot] = home_vertical.areas(area_home.inner(Margin::new(1, 1)));

        // Grid-layout für die Wohnung
        let top_horizontal = Layout::horizontal([Min(5), Min(5)]);
        let mid_horizontal = Layout::horizontal([Min(5), Max(20)]);
        let bot_horizontal = Layout::horizontal([Max(20), Max(25), Min(5)]);
        let [area_office, area_couch] = top_horizontal.areas(area_top);
        let [area_floor, area_kitchen] = mid_horizontal.areas(are_mid);
        let [area_door, area_bath, area_bed] = bot_horizontal.areas(area_bot);

        let all_areas = vec![
            area_office,
            area_couch,
            area_floor,
            area_kitchen,
            area_door,
            area_bath,
            area_bed,
        ];

        // todo: Wenn ich diese Strucktur Hier verändere, aknn ich die Angezeigten Daten in der Whonungsskizze anpassen (Farben, an aus, Ausgewählt oder nicht ...)
        let rooms: [(Borders, Vec<(&str, Color)>); 7] = [
            (
                Borders::RIGHT,
                vec![("lampe1", Color::Cyan), ("Strom1", Color::Yellow)],
            ),
            (Borders::NONE, vec![("Schalter2", Color::Cyan)]),
            (
                Borders::TOP | Borders::BOTTOM | Borders::RIGHT,
                vec![("lampe2", Color::Green)],
            ),
            (Borders::TOP | Borders::BOTTOM, vec![]),
            (Borders::RIGHT, vec![("temp1", Color::Green)]),
            (
                Borders::RIGHT,
                vec![
                    ("lampe3", Color::Cyan),
                    ("temp", Color::Yellow),
                    ("humidity", Color::Yellow),
                ],
            ),
            (
                Borders::NONE,
                vec![("lampe4", Color::Green), ("lampe5", Color::Green)],
            ),
        ];

        let block_appartment = Block::default().borders(Borders::ALL).title("Wohnung");
        frame.render_widget(block_appartment, area_home);

        // Debug-Fenster

        let app_text = String::from(self.dbmsg_buf.join("\n"));
        let paragraph = Paragraph::new(app_text)
            .block(
                Block::default()
                    .title("Textfeld")
                    .border_type(BorderType::Plain),
            )
            .wrap(Wrap { trim: true }) // Umbruch aktivieren
            .scroll((0, 0)); // Hauptscrollwert

        frame.render_widget(paragraph, area_debug);

        for i in 0..7 {
            let area = all_areas[i];
            let room = &rooms[i];
            let count = RECT_COUNTS[i];

            // Ruft die zentrierende Zeichenfunktion auf
            self.draw_centered_paragraphs(frame, area, count, room);
        }

        if self.entities.len() <= 0 {
            self.entities.push(String::from("No Entitys found"));
        }

        //Meine Vision:
        // Sci-Fi Terminal - nach Zimmeraufteilung

        // Liste mit allen Entitäten erstellen
        let items: Vec<ListItem> = self
            .entities
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();

        let list =
            List::new(items)
                .block(Block::default().borders(Borders::ALL).title(
                    "Scrollbare Liste (↑/↓ zum Navigieren, 'ENTER', 'STRG' + 'q' zum Beenden)",
                ))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ")
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom);

        frame.render_stateful_widget(list, area_sel_list, &mut self.state);
    }

    fn draw_centered_paragraphs(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        count: usize,
        room: &(Borders, Vec<(&str, Color)>),
    ) {
        let (borders, content) = room;
        let block = Block::default()
            .borders(*borders)
            .border_type(BorderType::Plain);
        frame.render_widget(block, area);

        if content.len() == 0 {
            return;
        }

        let inner_contrains = Layout::vertical([
            Constraint::Min(0),              // Linke Füllung (nimmt den Rest)
            Constraint::Length(RECT_HEIGHT), // Fixe Breite des Paragraphen
            Constraint::Min(0),              // Rechte Füllung (nimmt den Rest)
        ]);
        let [_, inner_area, _] = inner_contrains.areas(area);

        let final_contrain = Layout::horizontal([
            Constraint::Min(0),             // Obere Füllung
            Constraint::Length(RECT_WIDTH), // Fixe Höhe des Paragraphen
            Constraint::Length(RECT_WIDTH),
            Constraint::Length(RECT_WIDTH),
            Constraint::Length(RECT_WIDTH),
            Constraint::Length(RECT_WIDTH),
            Constraint::Min(0), // Untere Füllung
        ]);
        let final_area: [Rect; 7] = final_contrain.areas(inner_area);

        for i in 0..content.len() {
            let (msg, color) = content[i];

            let paragraph = Paragraph::new(Text::raw(msg))
                .style(Style::default().bg(color).fg(Color::White))
                // Zentriere den Text im Feld (optional, aber nützlich für Text)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, final_area[i + 1]);
        }
    }

    async fn handle_events(&mut self, event: Event) {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key).await,
            _ => {}
        }
    }

    async fn on_key_event(&mut self, key: KeyEvent) {
        match self.input_mode {
            InputModeE::NORMAL => match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => self.stop().await, // Shutdown
                (KeyModifiers::CONTROL, KeyCode::Char('r')) => self.reset().await, // Start
                (_, KeyCode::Up) => self.state.select_previous(),                 // Start
                (_, KeyCode::Down) => self.state.select_next(),                   // Start
                (_, KeyCode::Enter) => self.send_to_ha(UserMsg::DEBUG).await,     // Start
                _ => {}
            },
            InputModeE::EDIT_SPEEDUP => match (key.modifiers, key.code) {
                (_, k) => self.put_char(k),
            },
            InputModeE::EDIT_DURATION => match (key.modifiers, key.code) {
                (_, k) => self.put_char(k),
            },
        }
    }

    fn put_char(&mut self, k: KeyCode) {}

    async fn send_to_ha(&mut self, msg: UserMsg) {
        //self.to_ha.send(msg).await;
    //TODO? was wollte ich hier machen?
    }

    async fn stop(&mut self) {
        ratatui::restore();
        println!("Stopping TUI-APP...");
        self.send_to_ha(UserMsg::TERMINATE).await;
        self.running = false;
    }

    async fn reset(&mut self) {
        self.send_to_ha(UserMsg::RESET).await;
    }
}
