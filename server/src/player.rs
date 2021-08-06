use std::sync::Arc;

use serde::Serialize;

use tokio::sync::mpsc::{error::SendError, Sender};
use tokio_tungstenite::tungstenite::Message;

use crate::host::PlayerRoundEnd;

#[derive(Serialize, Debug)]
pub struct Player {
    #[serde(rename = "userName")]
    pub user_name: Arc<String>,
    pub points: usize,
    pub streak: usize,
    #[serde(skip)]
    pub player_sender: Sender<Message>,
    #[serde(skip)]
    pub played: bool,
    #[serde(skip)]
    pub player_round_end: Option<PlayerRoundEnd>,
}

impl Player {
    pub fn new(user_name: Arc<String>, player_sender: Sender<Message>) -> Self {
        Self {
            user_name,
            points: 0,
            streak: 0,
            player_sender,
            played: false,
            player_round_end: None,
        }
    }
    pub async fn send(
        &mut self,
        position: usize,
        behind: Option<Arc<String>>,
    ) -> Result<(), SendError<Message>> {
        self.played = false;

        let player_round_end = if let Some(mut player_round_end) = self.player_round_end.take() {
            player_round_end.position = position;
            player_round_end.behind = behind;

            player_round_end
        } else {
            self.streak = 0;
            PlayerRoundEnd {
                event: "questionEnd",
                correct: false,
                points_this_round: 0,
                points_total: self.points,
                streak: self.streak,
                position,
                behind,
            }
        };

        self.player_sender
            .send(Message::Text(
                serde_json::to_string(&player_round_end).unwrap(),
            ))
            .await
    }
}
