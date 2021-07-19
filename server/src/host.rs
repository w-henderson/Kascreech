use std::iter::repeat_with;

use serde::{Deserialize, Serialize};

use crate::{err::FailResponse, game::Game, Read, Write, GAMES};

use futures::{select, FutureExt, SinkExt, StreamExt};

use tokio_tungstenite::tungstenite::Message;

pub async fn host_command(s: &str, write: &mut Write, read: &mut Read) {
    let host_request = serde_json::from_str::<HostRequest>(s).unwrap();

    let (sender, mut receiver) = tokio::sync::mpsc::channel(5);

    match Game::new(
        &format!("https://play.kahoot.it/rest/kahoots/{}", host_request.id),
        sender,
    ) {
        Ok(game) => {
            let game_id: String = repeat_with(|| fastrand::digit(10)).take(6).collect();

            let message =
                Message::Text(serde_json::to_string(&game.make_response(&game_id)).unwrap());
            write.send(message).await.unwrap();

            GAMES.insert(game_id, game);
        }
        Err(e) => {
            let message = Message::Text(serde_json::to_string(&FailResponse::new(e)).unwrap());

            write.send(message).await.unwrap();
        }
    }

    loop {
        select! {
            recv = receiver.recv().fuse() => {
                if let Some(new_player) = recv {
                    let message = Message::Text(serde_json::to_string(&PlayerJoined {
                        event: "newPlayer", player_name: &new_player
                    }).unwrap());

                    write.send(message).await.unwrap();
                }
            },
            message = read.next().fuse() => {
                if let Some(Ok(message)) = message {
                    if let Ok(s) = message.to_text() {
                        if let Ok(request) = serde_json::from_str::<HostStartGame>(s) {
                            if request.command == "start" {
                                break
                            }
                        }
                    }
                }
            },
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
    fn make_response<'a>(&'a self, game_id: &'a str) -> SuccessResponse<'a> {
        SuccessResponse {
            success: true,
            game_id,
            game_name: &self.title,
            question_count: self.questions.len(),
        }
    }
}

#[derive(Deserialize)]
struct HostStartGame<'a> {
    command: &'a str,
}

#[derive(Serialize)]
struct PlayerJoined<'a> {
    event: &'static str,
    #[serde(rename = "playerName")]
    player_name: &'a str,
}
