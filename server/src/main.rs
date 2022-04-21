mod api;
mod database;
mod err;
mod host;
mod mon;
mod player;
mod types;

use database::DatabaseGame;
use err::{FailResponse, KascreechError};
use types::{ClientStatus, Game, GamePhase};

use humphrey::handlers::serve_dir;
use humphrey::monitor::event::{Event, EventLevel, EventType};
use humphrey::monitor::MonitorConfig;
use humphrey::App;

use humphrey_ws::async_app::AsyncSender;
use humphrey_ws::{async_websocket_handler, AsyncStream, AsyncWebsocketApp, Message};

use humphrey_json::Value;

use jasondb::Database;

use std::collections::HashMap;
use std::env::args;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;

pub struct AppState {
    clients: RwLock<HashMap<SocketAddr, ClientStatus>>,
    games: Mutex<HashMap<String, Game>>,
    database: Arc<Mutex<Database<DatabaseGame>>>,
    global_sender: Mutex<Option<AsyncSender>>,
    event_tx: Mutex<Sender<Event>>,
}

pub struct HumphreyAppState {
    database: Arc<Mutex<Database<DatabaseGame>>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &'static str = Box::leak(Box::new(
        args()
            .nth(2)
            .unwrap_or_else(|| "../client/build".to_string()),
    ));

    let (event_tx, event_rx) = channel();

    let db: Arc<Mutex<Database<DatabaseGame>>> = Arc::new(Mutex::new(
        Database::new("db.jdb")?
            .with_index("name")?
            .with_index("author")?
            .with_index("featured")?,
    ));

    let ws_app: AsyncWebsocketApp<AppState> = AsyncWebsocketApp::new_unlinked_with_config(
        AppState {
            clients: Default::default(),
            games: Default::default(),
            database: db.clone(),
            global_sender: Default::default(),
            event_tx: Mutex::new(event_tx.clone()),
        },
        32,
    )
    .with_connect_handler(connect_handler)
    .with_disconnect_handler(disconnect_handler)
    .with_message_handler(message_handler_internal);

    let sender = ws_app.sender();
    *ws_app.get_state().global_sender.lock().unwrap() = Some(sender);

    let humphrey_app: App<HumphreyAppState> =
        App::new_with_config(32, HumphreyAppState { database: db })
            .with_route("/api/v1/import", api::import)
            .with_route("/api/v1/featured", api::featured)
            .with_route("/api/v1/list", api::list)
            .with_route("/api/v1/search", api::search)
            .with_path_aware_route("/*", serve_dir(path))
            .with_websocket_route("/", async_websocket_handler(ws_app.connect_hook().unwrap()))
            .with_monitor(MonitorConfig::new(event_tx).with_subscription_to(EventLevel::Info));

    spawn(move || {
        humphrey_app
            .run(args().nth(1).unwrap_or_else(|| "0.0.0.0:80".to_string()))
            .unwrap()
    });

    spawn(move || mon::monitor(event_rx));

    ws_app.run();

    Ok(())
}

fn connect_handler(stream: AsyncStream, state: Arc<AppState>) {
    let mut clients = state.clients.write().unwrap();
    clients.insert(stream.peer_addr(), ClientStatus::Loading);

    let log = state.event_tx.lock().unwrap();
    log.send(
        Event::new(EventType::RequestServedSuccess)
            .with_peer(stream.peer_addr())
            .with_info("Kascreech: client connected"),
    )
    .ok();
}

fn disconnect_handler(stream: AsyncStream, state: Arc<AppState>) {
    let status = {
        let mut clients = state.clients.write().unwrap();
        clients.remove(&stream.peer_addr()).unwrap()
    };

    if let ClientStatus::Playing(game_id) = status {
        let mut games = state.games.lock().unwrap();
        let game = games.get_mut(&game_id).unwrap();
        game.players.remove(&stream.peer_addr());
    }

    let log = state.event_tx.lock().unwrap();
    log.send(
        Event::new(EventType::RequestServedSuccess)
            .with_peer(stream.peer_addr())
            .with_info("Kascreech: client disconnected"),
    )
    .ok();
}

fn message_handler_internal(mut stream: AsyncStream, message: Message, state: Arc<AppState>) {
    match message_handler(&mut stream, message, state.clone()) {
        Ok(_) => (),
        Err(e) => {
            stream.send(Message::new(humphrey_json::to_string(&e)));

            let log = state.event_tx.lock().unwrap();
            log.send(
                Event::new(EventType::RequestServedError)
                    .with_peer(stream.peer_addr())
                    .with_info(format!(
                        "Error: {}",
                        humphrey_json::to_string(&e.error_type)
                    )),
            )
            .ok();
        }
    }
}

fn message_handler(
    stream: &mut AsyncStream,
    message: Message,
    state: Arc<AppState>,
) -> Result<(), FailResponse> {
    let status = {
        let clients = state.clients.read().unwrap();
        clients.get(&stream.peer_addr()).unwrap().clone()
    };

    let json: Value = humphrey_json::from_str(message.text().unwrap())?;

    match status {
        ClientStatus::Loading => {
            let command = json
                .get("command")
                .ok_or_else(FailResponse::none_option)?
                .as_str()
                .ok_or_else(FailResponse::none_option)?;

            match command {
                "join" => player::join(stream, json, state),
                "host" => host::host(stream, json, state),
                _ => Err(FailResponse::new(
                    KascreechError::InvalidCommand,
                    Some("Only acceptable commands in this context are `join` and `host`".into()),
                )),
            }
        }

        ClientStatus::Playing(game_id) => {
            let game_phase = get_game_phase(&game_id, &state);
            player::handle_message(stream, json, state, game_id, game_phase)
        }

        ClientStatus::Hosting(game_id) => {
            let game_phase = get_game_phase(&game_id, &state);
            host::handle_message(stream, json, state, game_id, game_phase)
        }
    }
}

fn get_game_phase(game_id: &str, state: &Arc<AppState>) -> GamePhase {
    let games = state.games.lock().unwrap();
    let game = games.get(game_id).unwrap();
    game.phase
}

pub fn quiet_assert(condition: bool) -> Result<(), FailResponse> {
    if !condition {
        Err(FailResponse::new(
            KascreechError::InvalidCommand,
            Some("Command not valid at this time".into()),
        ))
    } else {
        Ok(())
    }
}
