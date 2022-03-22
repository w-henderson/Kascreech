#![feature(once_cell)]

mod err;
mod host;
mod player;
mod types;

use err::{FailResponse, KascreechError};
use types::{ClientStatus, Game};

use humphrey::handlers::serve_dir;
use humphrey::App;

use humphrey_ws::{async_websocket_handler, AsyncStream, AsyncWebsocketApp, Message};

use humphrey_json::Value;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;

#[derive(Default)]
pub struct AppState {
    clients: RwLock<HashMap<SocketAddr, ClientStatus>>,
    games: Mutex<HashMap<String, Game>>,
}

fn main() {
    let ws_app: AsyncWebsocketApp<AppState> = AsyncWebsocketApp::new_unlinked()
        .with_connect_handler(connect_handler)
        .with_disconnect_handler(disconnect_handler)
        .with_message_handler(message_handler_internal);

    let humphrey_app: App<()> = App::new()
        .with_path_aware_route("/*", serve_dir("../client/build"))
        .with_websocket_route("/", async_websocket_handler(ws_app.connect_hook().unwrap()));

    spawn(move || humphrey_app.run("0.0.0.0:80").unwrap());

    ws_app.run();
}

fn connect_handler(stream: AsyncStream, state: Arc<AppState>) {
    let mut clients = state.clients.write().unwrap();
    clients.insert(stream.peer_addr(), ClientStatus::Loading);
}

fn disconnect_handler(_: AsyncStream, _: Arc<AppState>) {}

fn message_handler_internal(mut stream: AsyncStream, message: Message, state: Arc<AppState>) {
    match message_handler(&mut stream, message, state) {
        Ok(_) => (),
        Err(e) => stream.send(Message::new(humphrey_json::to_string(&e))),
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
                    KascreechError::UnrecognisedCommand,
                    Some("Only acceptable commands in this context are `join` and `host`".into()),
                )),
            }
        }
        ClientStatus::Playing(game_id) => Ok(()),
        ClientStatus::Hosting(game_id) => Ok(()),
    }
}
