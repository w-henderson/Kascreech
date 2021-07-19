use serde::{Deserialize, Serialize};

use crate::{
    err::{FailResponse, KascreechError},
    player::Player,
    Write, GAMES,
};

use futures::SinkExt;

use tokio_tungstenite::tungstenite::Message;

pub async fn join_command(s: &str, write: &mut Write) {
    let join_request = serde_json::from_str::<JoinRequest>(s).unwrap();

    if let Some(mut t) = GAMES.get_mut(join_request.game_id) {
        let added = if let std::collections::hash_map::Entry::Vacant(e) =
            t.players.entry(join_request.player_name.clone())
        {
            e.insert(Player::default());

            let message =
                Message::Text(serde_json::to_string(&SuccessResponse { success: true }).unwrap());
            write.send(message).await.unwrap();

            true
        } else {
            let message = Message::Text(
                serde_json::to_string(&FailResponse::new(KascreechError::NameAlreadyExists))
                    .unwrap(),
            );
            write.send(message).await.unwrap();

            false
        };

        if added {
            t.host_sender.send(join_request.player_name).await.unwrap();
        }
    } else {
        let message = Message::Text(
            serde_json::to_string(&FailResponse::new(KascreechError::GameNotFound)).unwrap(),
        );
        write.send(message).await.unwrap();
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
