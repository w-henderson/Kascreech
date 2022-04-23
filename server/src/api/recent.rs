use crate::api::COUNT;
use crate::HumphreyAppState;

use humphrey::http::headers::ResponseHeader;
use humphrey::http::{Request, Response, StatusCode};

use std::sync::Arc;

pub fn recent(_: Request, state: Arc<HumphreyAppState>) -> Response {
    let mut db = state.database.lock().unwrap();

    let games = db
        .iter()
        .rev()
        .flatten()
        .take(COUNT)
        .map(|(_, game)| game)
        .collect::<Vec<_>>();

    Response::empty(StatusCode::OK)
        .with_bytes(humphrey_json::to_string(&games))
        .with_header(ResponseHeader::ContentType, "application/json".into())
}
