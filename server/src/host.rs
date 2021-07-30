use std::iter::repeat_with;

use serde::{Deserialize, Serialize};

use tokio_stream::wrappers::ReceiverStream;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    command::Command,
    game::{Game, KahootAnswer},
    player::Player,
    Read, Write, GAMES,
};

use log::warn;

use futures::{select, FutureExt, SinkExt, StreamExt};

pub async fn host_command(host_request: &str, write: &mut Write, read: &mut Read) {
    let host_request = serde_json::from_str::<HostRequest>(host_request).unwrap();

    let (player_sender, player_receiver) = tokio::sync::mpsc::channel(5);
    let mut player_receiver = ReceiverStream::new(player_receiver).fuse();

    let (question_sender, question_receiver) = tokio::sync::mpsc::channel(5);
    let mut question_receiver = ReceiverStream::new(question_receiver).fuse();

    let game_id: String = repeat_with(|| fastrand::digit(10)).take(6).collect();

    // Creating a new game
    match Game::new(
        &format!("https://play.kahoot.it/rest/kahoots/{}", host_request.id),
        player_sender,
        question_sender,
    ) {
        Ok(game) => {
            let message =
                Message::Text(serde_json::to_string(&game.make_response(&game_id)).unwrap());
            write.send(message).await.unwrap();

            GAMES.insert(game_id.clone(), game);
        }
        Err(e) => {
            crate::send_error(write, &e).await.unwrap();

            return;
        }
    }

    // Wait for either new players to join,
    // or the server to start the game
    loop {
        select! {
            // A new player's joined
            recv = player_receiver.next() => {
                if let Some(new_player) = recv {
                    let message = Message::Text(serde_json::to_string(&PlayerJoined {
                        event: "newPlayer", player_name: &new_player
                    }).unwrap());

                    write.send(message).await.unwrap();
                }
            },
            // The host's began the game
            message = read.next().fuse() => {
                if let Some(Ok(message)) = message {
                    if let Ok(s) = message.to_text() {
                        if let Ok(request) = serde_json::from_str::<Command>(s) {
                            if request.command == "start" {
                                break
                            }
                            warn!("unrecognised command \"{}\" when it should only be \"start\"", request.command);
                        }
                    }
                }
            },
        }
    }

    let mut game = GAMES.get_mut(&game_id).unwrap();

    // The actual game loop
    loop {
        match game.questions.next() {
            // Handle the next question
            Some(next_question) => {
                let player_send_question = Message::Text(
                    serde_json::to_string(&PlayerSendQuestion {
                        event: "questionStart",
                        number_of_answers: next_question.choices.len(),
                    })
                    .unwrap(),
                );

                for player in &game.players {
                    player
                        .player_sender
                        .send(player_send_question.clone())
                        .await
                        .unwrap()
                }

                let host_send_question = Message::Text(
                    serde_json::to_string(&HostSendQuestion {
                        question: &next_question.question,
                        duration: next_question.time,
                        answers: &next_question.choices,
                    })
                    .unwrap(),
                );

                write.send(host_send_question).await.unwrap();

                // Waiting for players to send answers, or the host
                // to request the leaderboard
                loop {
                    select! {
                    // A player's sent an answer
                    recv = question_receiver.next() => {
                        if let Some(player_guess) = recv {
                            let player = game.players.iter_mut().find(|p| p.user_name == player_guess.user_name);
                            if let Some(player) = player {
                                // The player has not played this round yet
                                if !player.played {
                                    if next_question.choices.get(player_guess.index).is_some() {
                                        println!("Player {} has answered with index {}", player.user_name, player_guess.index)
                                    }
                                }
                                player.played = true;
                            }
                        }
                    },
                    // The host's requested a leaderboard
                    message = read.next().fuse() => {
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

                // Actually send the leaderboard
                game.players.sort_by(|a, b| a.score.cmp(&b.score));

                let leader_board_response = Message::Text(
                    serde_json::to_string(&LeaderBoardResponse {
                        leaderboard: &game.players,
                    })
                    .unwrap(),
                );

                write.send(leader_board_response).await.unwrap();

                // Waiting for the host to request the next question
                loop {
                    if let Some(Ok(message)) = read.next().await {
                        if let Ok(message) = message.to_text() {
                            if let Ok(host_request_next_question) =
                                serde_json::from_str::<Command>(message)
                            {
                                if host_request_next_question.command == "question" {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            // No questions remain, the game ends
            None => {
                game.players.sort_by(|a, b| a.score.cmp(&b.score));
                for (i, player) in game.players.iter().enumerate() {
                    let game_over = Message::Text(
                        serde_json::to_string(&GameOver {
                            event: "end",
                            position: i + 1,
                        })
                        .unwrap(),
                    );
                    player.player_sender.send(game_over).await.unwrap();
                }
                break;
            }
        }
    }

    println!("Finished a connection");
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
    fn make_response<'r>(&'r self, game_id: &'r str) -> SuccessResponse<'r> {
        SuccessResponse {
            success: true,
            game_id,
            game_name: &self.title,
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
struct PlayerSendQuestion<'a> {
    event: &'a str,
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
struct GameOver<'a> {
    event: &'a str,
    position: usize,
}
