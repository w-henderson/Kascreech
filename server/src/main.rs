#![warn(clippy::nursery, clippy::pedantic)]

mod command;
mod err;
mod game;
mod player;

mod host;
mod join;

use command::Command;
use err::{FailResponse, KascreechError, KascreechResult};
use game::{Game, Senders};

use simple_log::LogConfigBuilder;

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use dashmap::DashMap;

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};

use once_cell::sync::Lazy;

type Write = SplitSink<WebSocketStream<TcpStream>, Message>;
type Read = SplitStream<WebSocketStream<TcpStream>>;

static GAMES: Lazy<DashMap<String, Game>> = Lazy::new(DashMap::default);
static HOST_SENDERS: Lazy<DashMap<String, Senders>> = Lazy::new(DashMap::default);

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let log_config = LogConfigBuilder::builder()
        .path("log.log")
        .output_file()
        .level("info")
        .build();

    simple_log::new(log_config).unwrap();

    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:80".to_string());

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) -> KascreechResult<()> {
    let ws_stream = tokio_tungstenite::accept_async(stream).await?;

    let (mut write, mut read) = ws_stream.split();

    let message = read
        .next()
        .await
        .ok_or(FailResponse::new(KascreechError::FailedRead, None))??;

    let s = message.to_text()?;

    let command = serde_json::from_str::<Command>(s)?;

    let err = match command.command {
        "host" => host::host_command(s, &mut write, &mut read).await,
        "join" => join::join_command(s, &mut write, &mut read).await,
        _ => Err(FailResponse::new(
            KascreechError::UnrecognisedCommand,
            Some(command.command.to_string()),
        )),
    };

    if let Err(e) = err {
        log::error!("{}", e);

        write
            .send(Message::Text(serde_json::to_string(&e).unwrap()))
            .await
            .unwrap();
    }

    Ok(())
}
