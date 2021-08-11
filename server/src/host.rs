use std::{convert::TryInto, iter::repeat_with, sync::Arc};

use serde::{Deserialize, Serialize};

use tokio_stream::wrappers::ReceiverStream;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    command::Command,
    err::{FailResponse, KascreechError, KascreechResult},
    game::{Game, KahootAnswer},
    player::Player,
    Read, Senders, Write, GAMES, HOST_SENDERS,
};

use log::warn;

use futures::{select, SinkExt, StreamExt};

pub async fn host_command(
    host_request: &str,
    write: &mut Write,
    read: &mut Read,
) -> KascreechResult<()> {
    let game_id = generate_game_id();

    let to_return = host_command_internal(game_id.clone(), host_request, write, read).await;

    if let Some(game) = GAMES.get(&game_id) {
        for p in &game.players {
            p.player_sender.send(Message::Close(None)).await.unwrap();
        }
    }

    GAMES.remove(&game_id);
    HOST_SENDERS.remove(&game_id);

    to_return
}

fn generate_game_id() -> String {
    repeat_with(|| fastrand::digit(10)).take(6).collect()
}

async fn host_command_internal(
    game_id: String,
    host_request: &str,
    write: &mut Write,
    read: &mut Read,
) -> KascreechResult<()> {
    let mut read = read.fuse();

    let host_request = serde_json::from_str::<HostRequest>(host_request)?;

    let (player_sender, player_receiver) = tokio::sync::mpsc::channel(5);
    let mut player_receiver = ReceiverStream::new(player_receiver).fuse();

    let (question_sender, question_receiver) = tokio::sync::mpsc::channel(5);
    let mut question_receiver = ReceiverStream::new(question_receiver).fuse();

    // Creating a new game
    let game = Game::new(
        &format!("https://play.kahoot.it/rest/kahoots/{}", host_request.id),
        player_sender,
    )?;

    let message = Message::Text(serde_json::to_string(&game.make_response(&game_id))?);
    write.send(message).await?;

    GAMES.insert(game_id.clone(), game);

    HOST_SENDERS.insert(game_id.clone(), Senders::new(question_sender));

    // Wait for either new players to join,
    // or the server to start the game
    loop {
        select! {
            // A new player's joined
            recv = player_receiver.next() => {
                if let Some(new_player) = recv {
                    let message = Message::Text(serde_json::to_string(&PlayerJoined {
                        event: "newPlayer", player_name: &new_player
                    })?);

                    write.send(message).await?;
                }
            },
            // The host's began the game
            message = read.next() => {
                if let Some(Ok(message)) = message {
                    if let Ok(s) = message.to_text() {
                        if let Ok(request) = serde_json::from_str::<Command>(s) {
                            if request.command == "start" {
                                break
                            }
                            warn!("Unrecognised command \"{}\" when it should only be \"start\"", request.command);
                        }
                    }
                }
            },
        }
    }

    let mut game = GAMES.get_mut(&game_id).unwrap();
    HOST_SENDERS.get_mut(&game_id).unwrap().in_progress = true;

    // The actual game loop
    while let Some(next_question) = game.questions.next() {
        let player_send_question = Message::Text(serde_json::to_string(&PlayerSendQuestion {
            event: "questionStart",
            number_of_answers: next_question.choices.len(),
        })?);

        for player in &game.players {
            player
                .player_sender
                .send(player_send_question.clone())
                .await
                .unwrap();
        }

        let host_send_question = Message::Text(serde_json::to_string(&HostSendQuestion {
            question: &next_question.question,
            duration: next_question.time,
            answers: &next_question.choices,
        })?);

        write.send(host_send_question).await?;

        let mut points = Points::new(game.players.len().try_into().unwrap());

        // The host is now ready to accept guesses
        HOST_SENDERS.get_mut(&game_id).unwrap().receiving = true;

        // Waiting for players to send answers, or the host
        // to request the leaderboard
        loop {
            select! {
            // A player's sent an answer
            recv = question_receiver.next() => {
                if let Some(player_guess) = recv {
                    // Checking the player exists
                    if let Some(player) = game.players.iter_mut().find(|p| p.user_name == player_guess.user_name) {
                        // The player has not played this round yet
                        if !player.played {
                            // Checking the guess is valid
                            if let Some(choice) = next_question.choices.get(player_guess.index) {
                                let (points_this_round, streak) = if choice.correct {
                                    let add_points = points.next_points();

                                    let points_this_round = <u16 as Into<usize>>::into(add_points);

                                    player.points += points_this_round;
                                    player.streak += 1;

                                    (points_this_round, player.streak)
                                } else {
                                    player.streak = 0;
                                    (0, 0)
                                };

                                player.player_round_end = Some(
                                    PlayerRoundEnd {
                                        event: "questionEnd",
                                        correct: choice.correct,
                                        points_this_round,
                                        points_total: player.points,
                                        streak,
                                        position: 0,
                                        behind: None,
                                    }
                                );
                            }
                        }
                        player.played = true;
                    }
                }
            },
            // The host's requested a leaderboard
            message = read.next() => {
                if let Some(Ok(message)) = message {
                        if let Ok(message) = message.to_text() {
                            if let Ok(host_request_leaderboard) =
                                serde_json::from_str::<Command>(message)
                            {
                                if host_request_leaderboard.command == "leaderboard" {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        HOST_SENDERS
            .get_mut(&game_id)
            .ok_or(FailResponse::new(KascreechError::GameNotFound, None))?
            .receiving = false;

        // Actually send the leaderboard
        game.players.sort_by(|a, b| a.points.cmp(&b.points));

        let leader_board_response = Message::Text(serde_json::to_string(&LeaderBoardResponse {
            leaderboard: &game.players,
        })?);

        {
            let player_len = game.players.len();
            let mut player_peek = game.players.iter_mut().enumerate().peekable();
            while let Some((pos, player)) = player_peek.next() {
                let behind = player_peek.peek().map(|p| p.1.user_name.clone());
                player.send(player_len - pos, behind).await.unwrap();
            }
        }

        write.send(leader_board_response).await?;

        // If no questions remain, break out of the game loop
        if game.questions.len() == 0 {
            break;
        }

        // Waiting for the host to request the next question
        loop {
            if let Some(Ok(message)) = read.next().await {
                if let Ok(message) = message.to_text() {
                    if let Ok(host_request_next_question) = serde_json::from_str::<Command>(message)
                    {
                        if host_request_next_question.command == "question" {
                            break;
                        }
                    }
                }
            } else {
                // If the host disconnects, close the thread
                return Ok(());
            }
        }
    }

    // No questions remain, the game ends
    game.players.sort_by(|b, a| a.points.cmp(&b.points));
    for (i, player) in game.players.iter().enumerate() {
        let game_over = Message::Text(serde_json::to_string(&GameOver {
            event: "end",
            position: i + 1,
        })?);
        player.player_sender.send(game_over).await.unwrap();
    }

    Ok(())
}

struct Points {
    placement: i32,
    num_of_players: i32,
}

impl Points {
    const fn new(num_of_players: i32) -> Self {
        Self {
            placement: 0,
            num_of_players,
        }
    }
    fn next_points(&mut self) -> u16 {
        let power = -(self.placement - 1) / (self.num_of_players - 1).max(1);

        self.placement += 1;

        (1000. * 1.5_f64.powi(power)) as u16
    }
}

#[derive(Debug, Serialize)]
pub struct PlayerRoundEnd {
    pub event: &'static str,
    pub correct: bool,
    #[serde(rename = "pointsThisRound")]
    pub points_this_round: usize,
    #[serde(rename = "pointsTotal")]
    pub points_total: usize,
    pub streak: usize,
    pub position: usize,
    pub behind: Option<Arc<String>>,
}

#[derive(Deserialize)]
struct HostRequest<'a> {
    id: &'a str,
}

#[derive(Serialize)]
struct SuccessResponse<'a> {
    success: bool,
    #[serde(rename = "gameId")]
    game_id: &'a str,
    #[serde(rename = "gameName")]
    game_name: &'a str,
    #[serde(rename = "questionCount")]
    question_count: usize,
}

impl Game {
    fn make_response<'a>(&'a self, game_id: &'a str) -> SuccessResponse<'a> {
        SuccessResponse {
            success: true,
            game_id,
            game_name: &self.game_code,
            question_count: self.questions.len(),
        }
    }
}

#[derive(Serialize)]
struct PlayerJoined<'a> {
    event: &'a str,
    #[serde(rename = "playerName")]
    player_name: &'a str,
}

#[derive(Serialize)]
struct PlayerSendQuestion {
    event: &'static str,
    #[serde(rename = "numberOfAnswers")]
    number_of_answers: usize,
}

#[derive(Serialize)]
struct HostSendQuestion<'a> {
    question: &'a str,
    duration: usize,
    answers: &'a [KahootAnswer],
}

#[derive(Serialize)]
struct LeaderBoardResponse<'a> {
    leaderboard: &'a [Player],
}

#[derive(Serialize)]
struct GameOver {
    event: &'static str,
    position: usize,
}
