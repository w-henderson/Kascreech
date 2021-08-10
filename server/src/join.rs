use std::sync::Arc;

use serde::{Deserialize, Serialize};

use tokio_stream::wrappers::ReceiverStream;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    err::FailResponse, player::Player, KascreechError, KascreechResult, Read, Write, GAMES,
    HOST_SENDERS,
};

use log::warn;

use futures::{select, SinkExt, StreamExt};

pub async fn join_command(
    join_request: &str,
    write: &mut Write,
    read: &mut Read,
) -> KascreechResult<()> {
    let mut read = read.fuse();

    let join_request = serde_json::from_str::<JoinRequest>(join_request)?;

    let JoinRequest {
        game_id,
        player_name,
    } = join_request;

    let player_name = Arc::new(player_name);

    // Checks if the game has already started
    if let Some(sender) = HOST_SENDERS.get(game_id) {
        if sender.in_progress {
            // Treats "game in progress" as "game not found" for the sake of the client
            let e = FailResponse::new(KascreechError::GameNotFound, None);

            crate::send_error!(write, e);
        }
    }

    let mut receiver = if let Some(mut game) = GAMES.get_mut(game_id) {
        // Checks if the player's name already exists
        let already_exists = game.players.iter().any(|p| p.user_name == player_name);

        if already_exists {
            let e = FailResponse::new(KascreechError::NameAlreadyExists, None);

            crate::send_error!(write, e);
        }

        let (player_sender, receiver) = tokio::sync::mpsc::channel(5);

        game.players
            .push(Player::new(Arc::clone(&player_name), player_sender));

        let message = Message::Text(serde_json::to_string(&SuccessResponse { success: true })?);

        write.send(message).await?;

        game.player_sender.send(player_name.clone()).await.unwrap();

        ReceiverStream::new(receiver).fuse()
    } else {
        let e = FailResponse::new(KascreechError::GameNotFound, None);

        crate::send_error!(write, e);
    };

    // Wait for either something to send,
    // or the server to start the game
    loop {
        select! {
            // Something to send to the client from
            // the host
            recv = receiver.next() => {
                if let Some(message) = recv {
                    if let Message::Close(_) = message {
                        break
                    }
                    write.send(message).await?;
                }
            },
            // An answer's been read from the client
            message = read.next() => {
                if let Some(Ok(message)) = message {
                    let message = message.into_data();

                    if let Ok(player_guess) = serde_json::from_slice::<PrivatePlayerGuess>(&message) {
                        if player_guess.command == "guess" {
                            let player_guess = PlayerGuess {
                                user_name: Arc::clone(&player_name),
                                index: player_guess.index
                            };

                            let senders = HOST_SENDERS.get(game_id).unwrap();

                            if senders.receiving {
                                senders.guess_sender.send(player_guess).await.unwrap();
                            }
                        } else {
                            warn!("Unrecognised command \"{}\" when it should only be \"guess\"", player_guess.command);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// The initial message a possible client should send
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

/// A client's guess
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
