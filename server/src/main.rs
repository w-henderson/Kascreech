#![warn(clippy::nursery, clippy::pedantic)]
#![feature(once_cell)]

mod command;
mod err;
mod game;

mod new_game;

use command::Command;
use game::Game;
use new_game::host_command;

use dashmap::DashMap;

use futures::{stream::SplitSink, StreamExt};

use once_cell::sync::Lazy;

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

type Write = SplitSink<WebSocketStream<TcpStream>, Message>;

static GAMES: Lazy<DashMap<String, Game>> = Lazy::new(|| DashMap::default());

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (mut write, mut read) = ws_stream.split();

    if let Some(Ok(message)) = read.next().await {
        if let Ok(s) = message.to_text() {
            if let Ok(command) = serde_json::from_str::<Command>(s) {
                match command.command {
                    "host" => host_command(s, &mut write).await,
                    _ => {
                        eprintln!("Unknown command type {}", command.command);
                        todo!("Error handling for incorrect command type");
                    }
                }
            }
        }
    }
}
