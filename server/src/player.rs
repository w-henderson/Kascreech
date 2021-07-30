use std::sync::Arc;

use serde::Serialize;

use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

#[derive(Serialize)]
pub struct Player {
    pub user_name: Arc<String>,
    pub score: usize,
    pub streak: usize,
    #[serde(skip)]
    pub player_sender: Sender<Message>,
    #[serde(skip)]
    pub played: bool,
}

impl Player {
    pub fn new(user_name: Arc<String>, player_sender: Sender<Message>) -> Self {
        Self {
            user_name,
            score: 0,
            streak: 0,
            player_sender,
            played: false,
        }
    }
}
