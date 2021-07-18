use std::iter::repeat_with;

use crate::{game::Game, Write, GAMES};

use futures::SinkExt;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

pub async fn host_command(s: &str, write: &mut Write) {
    let host_request = serde_json::from_str::<HostRequest>(s).unwrap();

    let game = Game::from_url(&format!(
        "https://play.kahoot.it/rest/kahoots/{}",
        host_request.id
    ))
    .unwrap();

    let game_id: String = repeat_with(|| fastrand::digit(10)).take(6).collect();

    let message = Message::Text(serde_json::to_string(&game.make_response(&game_id)).unwrap());
    write.send(message).await.unwrap();

    GAMES.insert(game_id, game).unwrap();
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
