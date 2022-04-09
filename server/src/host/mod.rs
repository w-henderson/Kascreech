mod kahoot_api;
mod not_once_cell;
mod points;

use kahoot_api::{generate_id, get_kahoot};

use crate::err::{FailResponse, KascreechError};
use crate::types::{ClientStatus, Game, GamePhase, Player, PlayerRoundEnd};
use crate::{quiet_assert, AppState};

use humphrey::monitor::event::{Event, EventType};

use humphrey_ws::async_app::AsyncSender;
use humphrey_ws::{AsyncStream, Message};

use humphrey_json::prelude::*;
use humphrey_json::Value;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;

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

    let kahoot = get_kahoot(kahoot_id)
        .map_err(|e| FailResponse::new(KascreechError::KahootGameNotFound, Some(e.to_string())))?;

    let id = generate_id();
    let len = kahoot.questions.len();

    let game = Game {
        id: id.clone(),
        questions: kahoot_api::kahoot_questions_to_normal_questions(kahoot.questions).into_iter(),
        phase: GamePhase::Lobby,
        players: HashMap::new(),
        host: stream.peer_addr(),
        correct_answers: Vec::new(),
        question_start_time: 0,
    };

    let mut games = state.games.lock().unwrap();
    let mut clients = state.clients.write().unwrap();

    games.insert(game.id.clone(), game);
    clients.insert(stream.peer_addr(), ClientStatus::Hosting(id.clone()));

    let response = json!({
        "success": true,
        "gameId": (id.clone()),
        "gameName": (kahoot.title),
        "questionCount": len
    });

    stream.send(Message::new(response.serialize()));

    let log = state.event_tx.lock().unwrap();
    log.send(
        Event::new(EventType::RequestServedSuccess)
            .with_peer(stream.peer_addr())
            .with_info(format!("Kascreech: game hosted with ID {}", id)),
    )
    .ok();

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
            let endgame = game.questions.len() == 0;
            quiet_assert(command == "leaderboard")?;
            answer_command(stream, game, global_sender, endgame)?;

            if endgame {
                games.remove(&game_id);
            }

            Ok(())
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
    game.question_start_time = UNIX_EPOCH.elapsed().unwrap().as_millis();

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

    let question_duration = UNIX_EPOCH.elapsed().unwrap().as_millis() - game.question_start_time;

    for player in game.players.values_mut() {
        let correct = player
            .played
            .map(|guess| game.correct_answers.contains(&guess))
            .unwrap_or(false);

        let points_this_round = points::calculate_points(
            game.question_start_time,
            player.answer_time,
            question_duration,
            correct,
            player.streak,
        );

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
