use crate::err::{FailResponse, KascreechError};
use crate::types::{ClientStatus, GamePhase, Player};
use crate::AppState;

use humphrey_ws::{AsyncStream, Message};

use humphrey_json::prelude::*;
use humphrey_json::Value;

use std::sync::Arc;

pub fn join(
    stream: &mut AsyncStream,
    json: Value,
    state: Arc<AppState>,
) -> Result<(), FailResponse> {
    let id = json
        .get("gameId")
        .ok_or_else(FailResponse::none_option)?
        .as_str()
        .ok_or_else(FailResponse::none_option)?;

    let name = json
        .get("playerName")
        .ok_or_else(FailResponse::none_option)?
        .as_str()
        .ok_or_else(FailResponse::none_option)?;

    let mut games = state.games.lock().unwrap();

    let game = games.get_mut(id).ok_or_else(|| {
        FailResponse::new(
            KascreechError::GameNotFound,
            Some("The specified game ID does not exist".into()),
        )
    })?;

    let game_started = game.phase != GamePhase::Lobby;

    let name_taken = game
        .players
        .iter()
        .any(|(_, player)| player.name.to_ascii_lowercase() == name.to_ascii_lowercase());

    if game_started {
        Err(FailResponse::new(
            KascreechError::GameNotFound,
            Some("The game has already started".into()),
        ))
    } else if name_taken {
        Err(FailResponse::new(
            KascreechError::NameAlreadyExists,
            Some("The specified name is already taken".into()),
        ))
    } else {
        game.players.insert(
            stream.peer_addr(),
            Player {
                name: name.to_string(),
                points: 0,
                streak: 0,
                played: false,
                player_round_end: None,
            },
        );

        let host = game.host;

        drop(games);

        let mut clients = state.clients.write().unwrap();
        clients.insert(stream.peer_addr(), ClientStatus::Playing(id.to_string()));

        let response = json!({
            "success": true
        });

        stream.send(Message::new(response.serialize()));

        let sender = state.global_sender.lock().unwrap();
        let sender_ref = sender.as_ref().unwrap();

        let host_message = json!({
            "event": "newPlayer",
            "playerName": name
        });

        sender_ref.send(host, Message::new(host_message.serialize()));

        Ok(())
    }
}

pub fn handle_message(
    stream: &mut AsyncStream,
    json: Value,
    state: Arc<AppState>,
    game_id: String,
    game_phase: GamePhase,
) -> Result<(), FailResponse> {
    Ok(())
}
