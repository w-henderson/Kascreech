use crate::HumphreyAppState;

use humphrey::http::headers::ResponseHeader;
use humphrey::http::{Request, Response, StatusCode};

use jasondb::query;

use std::sync::Arc;

pub fn featured(_: Request, state: Arc<HumphreyAppState>) -> Response {
    let mut db = state.database.lock().unwrap();

    let featured_games = db
        .query(query!(featured == true))
        .unwrap()
        .flatten()
        .map(|(_, game)| game)
        .collect::<Vec<_>>();

    Response::empty(StatusCode::OK)
        .with_bytes(humphrey_json::to_string(&featured_games))
        .with_header(ResponseHeader::ContentType, "application/json".into())
}
