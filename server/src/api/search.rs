use crate::api::COUNT;
use crate::HumphreyAppState;

use humphrey::http::headers::ResponseHeader;
use humphrey::http::{Request, Response, StatusCode};

use humphrey_json::Value;

use jasondb::query;

use std::sync::Arc;

pub fn search(request: Request, state: Arc<HumphreyAppState>) -> Response {
    let query = request
        .content
        .and_then(|content| String::from_utf8(content).ok())
        .and_then(|content| Value::parse(&content).ok())
        .and_then(|json| {
            json.get("query")
                .and_then(|query| query.as_str())
                .map(|s| s.to_string())
        });

    if let Some(query_string) = query {
        let mut db = state.database.lock().unwrap();

        let query_string = query_string.to_ascii_lowercase();
        let query_string_2 = query_string.clone();

        let query = query!(name, move |name| {
            name.as_str()
                .map(|s| s.to_ascii_lowercase().contains(&query_string))
                .unwrap_or(false)
        }) | query!(author, move |author| {
            author
                .as_str()
                .map(|s| s.to_ascii_lowercase().contains(&query_string_2))
                .unwrap_or(false)
        });

        let games = db
            .query(query)
            .unwrap()
            .flatten()
            .take(COUNT)
            .map(|(_, game)| game)
            .collect::<Vec<_>>();

        Response::empty(StatusCode::OK)
            .with_bytes(humphrey_json::to_string(&games))
            .with_header(ResponseHeader::ContentType, "application/json".into())
    } else {
        Response::empty(StatusCode::BadRequest).with_bytes("Include a query field")
    }
}
