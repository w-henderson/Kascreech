use std::sync::Arc;

use serde::{Deserialize, Serialize};

use tokio_tungstenite::tungstenite::Message;

use crate::{player::Player, KascreechError, Read, Write, GAMES};

use futures::{select, FutureExt, SinkExt, StreamExt};

pub async fn join_command(join_request: &str, write: &mut Write, read: &mut Read) {
    let join_request = serde_json::from_str::<JoinRequest>(join_request).unwrap();

    let JoinRequest {
        game_id,
        player_name,
    } = join_request;

    let player_name = Arc::new(player_name);

    let mut receiver = match GAMES.get_mut(game_id) {
        Some(mut game) => {
            // Checks if the player's name already exists
            let already_exists = game.players.iter().any(|p| p.user_name == player_name);

            if already_exists {
                crate::send_error!(write, KascreechError::NameAlreadyExists).unwrap();

                return;
            }

            let (player_sender, receiver) = tokio::sync::mpsc::channel(5);

            game.players
                .push(Player::new(Arc::clone(&player_name), player_sender));

            let message =
                Message::Text(serde_json::to_string(&SuccessResponse { success: true }).unwrap());

            write.send(message).await.unwrap();

            game.player_sender
                .send(Arc::clone(&player_name))
                .await
                .unwrap();

            receiver
        }
        None => {
            crate::send_error!(write, KascreechError::GameNotFound).unwrap();

            return;
        }
    };

    // Wait for either something to send,
    // or the server to start the game
    loop {
        select! {
            // Something to send to the client from
            // the host
            recv = receiver.recv().fuse() => {
                if let Some(message) = recv {
                    write.send(message).await.unwrap();
                }
            },
            // An answer's been read from the client
            message = read.next().fuse() => {
                if let Some(Ok(message)) = message {
                    let message = message.into_data();

                    let index = if let Ok(player_guess) = serde_json::from_slice::<PrivatePlayerGuess>(&message) {
                        if player_guess.command == "guess" {
                            player_guess.index
                        } else {
                            continue
                        }
                    } else {
                        continue
                    };

                    let player_guess = PlayerGuess {
                        user_name: Arc::clone(&player_name), index
                    };

                    let game = GAMES.get(game_id).unwrap();

                    if game.receiving {
                        game.question_sender.send(player_guess).await.unwrap();
                    } else {
                        crate::send_error!(write, KascreechError::NotReceivingAnswers).unwrap();
                    }
                }
            },
        }
    }
}

#[derive(Deserialize)]
struct JoinRequest<'a> {
    #[serde(rename = "gameId")]
    game_id: &'a str,
    #[serde(rename = "playerName")]
    player_name: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
}

#[derive(Debug)]
pub struct PlayerGuess {
    pub user_name: Arc<String>,
    pub index: usize,
}

/// The struct that's actually received from the
/// client
#[derive(Deserialize, Debug)]
struct PrivatePlayerGuess<'a> {
    pub command: &'a str,
    pub index: usize,
}
