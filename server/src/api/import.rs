use crate::api::not_once_cell::NotOnceCell;
use crate::database::{DatabaseGame, DatabaseQuestion};
use crate::err::{FailResponse, KascreechError};
use crate::HumphreyAppState;

use humphrey::http::{Request, Response, StatusCode};
use humphrey::Client;

use humphrey_json::prelude::*;
use humphrey_json::Value;

use std::error::Error;
use std::sync::{Arc, Mutex};

static CLIENT: NotOnceCell<Mutex<Client>> = NotOnceCell::new();

pub fn import(request: Request, state: Arc<HumphreyAppState>) -> Response {
    import_inner(request, state).unwrap_or_else(|e| {
        let status = match e.error_type {
            KascreechError::FailedRead => StatusCode::InternalError,
            KascreechError::GameNotFound => StatusCode::NotFound,
            KascreechError::KahootGameNotFound => StatusCode::NotFound,
            KascreechError::UsernameAlreadyExists => StatusCode::Conflict,
            KascreechError::GameAlreadyExists => StatusCode::Conflict,
            KascreechError::InvalidCommand => StatusCode::BadRequest,
            KascreechError::UnknownError => StatusCode::InternalError,
        };

        Response::empty(status).with_bytes(humphrey_json::to_string(&e))
    })
}

fn import_inner(request: Request, state: Arc<HumphreyAppState>) -> Result<Response, FailResponse> {
    let json_error = FailResponse::new(
        KascreechError::InvalidCommand,
        Some("Request could not be parsed".to_string()),
    );

    let json = request
        .content
        .ok_or_else(|| json_error.clone())
        .and_then(|content| String::from_utf8(content).map_err(|_| json_error.clone()))
        .and_then(|content| Value::parse(&content).map_err(|_| json_error))?;

    let kahoot_id = json
        .get("id")
        .ok_or_else(FailResponse::none_option)?
        .as_str()
        .ok_or_else(FailResponse::none_option)?;

    let game = get_kahoot(kahoot_id)
        .map_err(|e| FailResponse::new(KascreechError::KahootGameNotFound, Some(e.to_string())))?;
    let id = game.id.clone();

    let mut db = state.database.lock().unwrap();

    if db.get(&id).is_ok() {
        return Err(FailResponse::new(
            KascreechError::GameAlreadyExists,
            Some("Game has already been imported".to_string()),
        ));
    }

    db.set(&id, game).map_err(|_| {
        FailResponse::new(
            KascreechError::UnknownError,
            Some("Database error".to_string()),
        )
    })?;

    let response = json!({
        "success": true,
        "gameId": (&id)
    });

    Ok(Response::empty(StatusCode::Created).with_bytes(response.serialize()))
}

fn get_kahoot(id: &str) -> Result<DatabaseGame, Box<dyn Error>> {
    if id.is_empty() {
        return Err("No Kahoot ID provided".into());
    }

    let mut client = CLIENT.get_or_init(|| Mutex::new(Client::new())).lock()?;
    let response = client
        .get(format!("https://play.kahoot.it/rest/kahoots/{}", id))?
        .send()?;
    let game = Value::parse(response.text().ok_or("Invalid response")?)?;

    let uuid = extract(&game, "uuid")?;
    let title = extract(&game, "title")?;
    let description = extract(&game, "description")?;
    let author = extract(&game, "creator_username")?;
    let image = extract(&game, "cover").ok();

    let mut questions = Vec::new();

    for question in game
        .get("questions")
        .ok_or("No questions")?
        .as_array()
        .ok_or("Invalid questions type")?
    {
        let question_type = extract(question, "type")?;

        if question_type == "quiz"
            || question_type == "survey"
            || question_type == "multiple_select_quiz"
        {
            if let Ok(question) = DatabaseQuestion::from_json(question) {
                questions.push(question);
            }
        }
    }

    if questions.is_empty() {
        return Err("No questions found".into());
    }

    Ok(DatabaseGame {
        id: format!("kahoot-{}", uuid),
        name: title,
        description,
        author,
        image,
        questions,
        plays: 0,
        kahoot: true,
        featured: false,
    })
}

fn extract(value: &Value, key: &str) -> Result<String, Box<dyn Error>> {
    value
        .get(key)
        .ok_or("Key not found")?
        .as_str()
        .ok_or_else(|| "Invalid value".into())
        .map(|s| s.to_string())
}
