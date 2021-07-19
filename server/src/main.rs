#![warn(clippy::nursery, clippy::pedantic)]

mod command;
mod err;
mod game;
mod player;

mod host;
mod join;

use command::Command;
use game::Game;

use log::{info, warn};
use simple_log::LogConfigBuilder;

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use dashmap::DashMap;

use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};

use once_cell::sync::Lazy;

type Write = SplitSink<WebSocketStream<TcpStream>, Message>;
type Read = SplitStream<WebSocketStream<TcpStream>>;

static GAMES: Lazy<DashMap<String, Game>> = Lazy::new(DashMap::default);

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let log_config = LogConfigBuilder::builder()
        .path("log.log")
        .output_file()
        .build();

    simple_log::new(log_config).unwrap();

    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:80".to_string());

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        info!("New connection");
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
                    "host" => host::host_command(s, &mut write, &mut read).await,
                    "join" => join::join_command(s, &mut write).await,
                    _ => {
                        warn!("Unknown command type {}", command.command);
                    }
                }
            }
        }
    }
}
