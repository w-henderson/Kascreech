mod kahoot_api;

use kahoot_api::{generate_id, get_kahoot};

use crate::err::{FailResponse, KascreechError};
use crate::types::{ClientStatus, Game};
use crate::AppState;

use humphrey_ws::{AsyncStream, Message};

use humphrey_json::prelude::*;
use humphrey_json::Value;

use std::collections::HashMap;
use std::sync::Arc;

pub fn host(
    stream: &mut AsyncStream,
    json: Value,
    state: Arc<AppState>,
) -> Result<(), FailResponse> {
    let kahoot_id = json
        .get("id")
        .ok_or_else(FailResponse::none_option)?
        .as_str()
        .ok_or_else(FailResponse::none_option)?;

    let kahoot = get_kahoot(kahoot_id).map_err(|_| {
        FailResponse::new(
            KascreechError::GameNotFound,
            Some("Kahoot game could not be loaded".into()),
        )
    })?;

    let id = generate_id();
    let len = kahoot.questions.len();

    let game = Game {
        id: id.clone(),
        questions: kahoot.questions.into_iter(),
        players: HashMap::new(),
        host: stream.peer_addr(),
    };

    let mut games = state.games.lock().unwrap();
    let mut clients = state.clients.write().unwrap();

    games.insert(game.id.clone(), game);
    clients.insert(stream.peer_addr(), ClientStatus::Hosting(id.clone()));

    let response = json!({
        "success": true,
        "gameId": id,
        "gameName": (kahoot.title),
        "questionCount": len
    });

    stream.send(Message::new(response.serialize()));

    Ok(())
}
