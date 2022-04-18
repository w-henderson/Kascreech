use crate::HumphreyAppState;

use humphrey::http::headers::ResponseHeader;
use humphrey::http::{Request, Response, StatusCode};

use humphrey_json::Value;

use std::sync::Arc;

const COUNT: usize = 10;

pub fn list(request: Request, state: Arc<HumphreyAppState>) -> Response {
    let offset = request
        .content
        .and_then(|content| String::from_utf8(content).ok())
        .and_then(|content| Value::parse(&content).ok())
        .and_then(|json| json.get("offset").and_then(|limit| limit.as_number()))
        .map(|limit| limit as usize);

    if let Some(offset) = offset {
        let mut db = state.database.lock().unwrap();

        let games = db
            .iter()
            .skip(offset)
            .flatten()
            .take(COUNT)
            .map(|(_, game)| game)
            .collect::<Vec<_>>();

        Response::empty(StatusCode::OK)
            .with_bytes(humphrey_json::to_string(&games))
            .with_header(ResponseHeader::ContentType, "application/json".into())
    } else {
        Response::empty(StatusCode::BadRequest).with_bytes("Include an offset field")
    }
}
