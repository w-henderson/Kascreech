#![feature(once_cell)]

mod err;
mod kahoot_api;
mod types;

use humphrey::handlers::serve_dir;
use humphrey::App;

use humphrey_ws::{async_websocket_handler, AsyncWebsocketApp};

use std::thread::spawn;

#[derive(Default)]
struct AppState {}

fn main() {
    let ws_app: AsyncWebsocketApp<AppState> = AsyncWebsocketApp::new_unlinked();

    let humphrey_app: App<()> = App::new()
        .with_path_aware_route("/*", serve_dir("../client/build"))
        .with_websocket_route("/", async_websocket_handler(ws_app.connect_hook().unwrap()));

    spawn(move || humphrey_app.run("0.0.0.0:80").unwrap());

    ws_app.run();
}
