mod kahoot_api;
mod not_once_cell;

use humphrey_ws::async_app::AsyncSender;
use kahoot_api::{generate_id, get_kahoot};

use crate::err::{FailResponse, KascreechError};
use crate::types::{ClientStatus, Game, GamePhase, Player, PlayerRoundEnd};
use crate::{quiet_assert, AppState};

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
        correct_answers: Vec::new(),
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
            answer_command(stream, game, global_sender, game.questions.len() == 0)
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
    let question = game
        .questions
        .next()
        .ok_or_else(FailResponse::none_option)?;

    let correct_answers: Vec<usize> = question
        .answers
        .iter()
        .enumerate()
        .filter(|(_, a)| a.correct)
        .map(|(i, _)| i)
        .collect();

    game.phase = GamePhase::Question;
    game.correct_answers = correct_answers;

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
    endgame: bool,
) -> Result<(), FailResponse> {
    game.phase = GamePhase::Leaderboard;

    for player in game.players.values_mut() {
        let correct = player
            .played
            .map(|guess| game.correct_answers.contains(&guess))
            .unwrap_or(false);
        let points_this_round = correct as usize * 800;

        let streak = if correct { player.streak + 1 } else { 0 };

        player.points += points_this_round;
        player.streak = streak;
        player.played = None;

        player.player_round_end = Some(PlayerRoundEnd {
            event: "questionEnd".into(),
            correct,
            points_this_round,
            points_total: player.points,
            streak: player.streak,
            position: 0,
            behind: None,
        });
    }

    let mut players_sorted: Vec<&mut Player> = game.players.values_mut().collect();
    players_sorted.sort_by_key(|p| p.points);

    let mut position = players_sorted.len();
    let mut behind = None;

    for player in &mut players_sorted {
        let stats = player.player_round_end.as_mut().unwrap();
        stats.position = position;
        stats.behind = behind;

        behind = Some(player.name.clone());
        position -= 1;
    }

    let leaderboard = json!({
        "leaderboard": (players_sorted.iter().map(|p| (**p).clone()).collect::<Vec<Player>>())
    })
    .serialize();

    stream.send(Message::new(leaderboard));

    let sender = global_sender.lock().unwrap();
    let sender_ref = sender.as_ref().unwrap();

    for (addr, player) in &game.players {
        let message = humphrey_json::to_string(player.player_round_end.as_ref().unwrap());

        sender_ref.send(*addr, Message::new(message));

        if endgame {
            let message = json!({
                "event": "end",
                "position": (player.player_round_end.as_ref().unwrap().position)
            })
            .serialize();
            sender_ref.send(*addr, Message::new(message));
        }
    }

    Ok(())
}
