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

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};
use tokio_tungstenite::{
    tungstenite::{
        protocol::{Role, WebSocketConfig},
        Message,
    },
    WebSocketStream,
};

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

const WEBSOCKET_CONFIG: WebSocketConfig = WebSocketConfig {
    max_send_queue: Some(5),
    max_message_size: Some(1 << 20),
    max_frame_size: Some(16 << 20),
    accept_unmasked_frames: false,
};

async fn accept_connection(stream: TcpStream) {
    if let Err(e) = accept_connection_internal(stream).await {
        log::error!("{}", e);
    }
}

async fn accept_connection_internal(mut stream: TcpStream) -> KascreechResult<()> {
    let mut buf = [0; 1000];
    let mut buf_reader = BufReader::new(&mut stream);

    let read = buf_reader.read(&mut buf).await.unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut req = httparse::Request::new(&mut headers);

    req.parse(&buf).unwrap();

    // If the stream is a websocket stream
    if let Some(header) = req.headers.iter().find(|h| h.name == "Sec-WebSocket-Key") {
        let mut key = std::str::from_utf8(header.value).unwrap().to_string();
        key.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        let sha = sha1::Sha1::from(key);

        let hash = sha.digest().bytes();
        let key = base64::encode(hash);

        let response = format!(
                "HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Accept: {}\r\n\r\n",
                key
            );

        stream.write(response.as_bytes()).await.unwrap();

        stream.flush().await.unwrap();

        let ws_stream =
            WebSocketStream::from_raw_socket(stream, Role::Server, Some(WEBSOCKET_CONFIG)).await;

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
        // Proxy the stream to localhost:8000
    } else {
        proxy(stream, &buf[..read]).await;
    }

    Ok(())
}

async fn proxy(mut inbound: TcpStream, already_read: &[u8]) {
    let mut outbound = TcpStream::connect("localhost:8000").await.unwrap();

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    wo.write_all(already_read).await.unwrap();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client).unwrap();
}
