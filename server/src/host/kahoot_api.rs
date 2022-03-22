use crate::types::KahootGame;

use humphrey::http::address::Address;
use humphrey::http::headers::RequestHeader;
use humphrey::http::method::Method;
use humphrey::http::{Request, Response};

use rustls::{Certificate, ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use rustls_native_certs::load_native_certs;

use std::collections::BTreeMap;
use std::error::Error;
use std::io::Write;
use std::lazy::SyncOnceCell;
use std::net::TcpStream;
use std::sync::Arc;

static CLIENT_CONFIG: SyncOnceCell<Arc<ClientConfig>> = SyncOnceCell::new();

pub fn get_kahoot(id: &str) -> Result<KahootGame, Box<dyn Error>> {
    let request = Request {
        method: Method::Get,
        uri: format!("/rest/kahoots/{}", id),
        query: String::new(),
        version: "HTTP/1.1".into(),
        headers: {
            let mut headers = BTreeMap::new();
            headers.insert(RequestHeader::Host, "play.kahoot.it".into());
            headers
        },
        content: None,
        address: Address::new("0.0.0.0:80")?,
    };

    let conn = ClientConnection::new(
        CLIENT_CONFIG.get_or_init(init_client_config).clone(),
        "play.kahoot.it".try_into()?,
    )?;
    let sock = TcpStream::connect("play.kahoot.it:443")?;
    let mut tls = StreamOwned::new(conn, sock);

    let bytes: Vec<u8> = request.into();
    tls.write_all(&bytes)?;

    let response = Response::from_stream(&mut tls)?;
    let body = String::from_utf8(response.body)?;
    let game: KahootGame = humphrey_json::from_str(body)?;

    Ok(game)
}

pub fn generate_id() -> String {
    let mut buf = [0u8; 4];
    getrandom::getrandom(&mut buf).expect("Failed to generate random ID");

    let id = u32::from_be_bytes(buf) % 1_000_000;

    format!("{:06}", id)
}

fn init_client_config() -> Arc<ClientConfig> {
    let mut roots = RootCertStore::empty();
    for cert in load_native_certs().expect("Failed to load native certs") {
        roots.add(&Certificate(cert.0)).unwrap();
    }

    let conf = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth();

    Arc::new(conf)
}
