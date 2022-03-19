use crate::types::{KahootGame, Question};

use humphrey::http::address::Address;
use humphrey::http::headers::RequestHeader;
use humphrey::http::method::Method;
use humphrey::http::{Request, Response};

use rustls::{Certificate, ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use rustls_native_certs::load_native_certs;

use std::collections::BTreeMap;
use std::io::Write;
use std::lazy::SyncOnceCell;
use std::net::TcpStream;
use std::sync::Arc;
use std::vec::IntoIter;

static CLIENT_CONFIG: SyncOnceCell<Arc<ClientConfig>> = SyncOnceCell::new();

pub fn get_kahoot(id: &str) -> IntoIter<Question> {
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
        address: Address::new("0.0.0.0:80").unwrap(),
    };

    let conn = ClientConnection::new(
        CLIENT_CONFIG.get_or_init(init_client_config).clone(),
        "play.kahoot.it".try_into().unwrap(),
    )
    .unwrap();
    let sock = TcpStream::connect("play.kahoot.it:443").unwrap();
    let mut tls = StreamOwned::new(conn, sock);

    let bytes: Vec<u8> = request.into();
    tls.write_all(&bytes).unwrap();

    let response = Response::from_stream(&mut tls).unwrap();
    let body = String::from_utf8(response.body).unwrap();
    let game: KahootGame = humphrey_json::from_str(body).unwrap();

    game.questions.into_iter()
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
