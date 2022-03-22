mod kahoot_api;

use humphrey_ws::async_app::AsyncSender;
use kahoot_api::{generate_id, get_kahoot};

use crate::err::{FailResponse, KascreechError};
use crate::types::{ClientStatus, Game, GamePhase};
use crate::AppState;

use humphrey_ws::{AsyncStream, Message};

use humphrey_json::prelude::*;
use humphrey_json::Value;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
        questions: kahoot_api::kahoot_questions_to_normal_questions(kahoot.questions).into_iter(),
        phase: GamePhase::Lobby,
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

pub fn handle_message(
    stream: &mut AsyncStream,
    json: Value,
    state: Arc<AppState>,
    game_id: String,
    game_phase: GamePhase,
) -> Result<(), FailResponse> {
    let command = json
        .get("command")
        .ok_or_else(FailResponse::none_option)?
        .as_str()
        .ok_or_else(FailResponse::none_option)?;

    let mut games = state.games.lock().unwrap();
    let game = games
        .get_mut(&game_id)
        .ok_or_else(FailResponse::none_option)?;

    let global_sender = &state.global_sender;

    match game_phase {
        GamePhase::Lobby => {
            quiet_assert(command == "start")?;
            question_command(stream, game, global_sender)
        }

        GamePhase::Question => {
            quiet_assert(command == "leaderboard")?;
            answer_command(stream, game, global_sender)
        }

        GamePhase::Leaderboard => {
            quiet_assert(command == "question")?;
            question_command(stream, game, global_sender)
        }
    }
}

fn question_command(
    stream: &mut AsyncStream,
    game: &mut Game,
    global_sender: &Mutex<Option<AsyncSender>>,
) -> Result<(), FailResponse> {
    game.phase = GamePhase::Question;

    let question = game
        .questions
        .next()
        .ok_or_else(FailResponse::none_option)?;

    let number_of_answers = question.answers.len();

    let host_response = humphrey_json::to_string(&question);
    let player_response = json!({
        "event": "questionStart",
        "numberOfAnswers": number_of_answers
    })
    .serialize();

    stream.send(Message::new(host_response));

    let sender = global_sender.lock().unwrap();
    let sender_ref = sender.as_ref().unwrap();

    for player in game.players.keys() {
        sender_ref.send(*player, Message::new(player_response.clone()));
    }

    Ok(())
}

fn answer_command(
    stream: &mut AsyncStream,
    game: &mut Game,
    global_sender: &Mutex<Option<AsyncSender>>,
) -> Result<(), FailResponse> {
    Ok(())
}

fn quiet_assert(condition: bool) -> Result<(), FailResponse> {
    if !condition {
        Err(FailResponse::new(
            KascreechError::UnrecognisedCommand,
            Some("Command not valid at this time".into()),
        ))
    } else {
        Ok(())
    }
}
