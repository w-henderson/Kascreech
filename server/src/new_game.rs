use serde::{Deserialize, Serialize};

use salvo::{async_trait, fn_handler, Request, Response};

use crate::game::Game;

#[fn_handler]
pub async fn new_game_handle(req: &mut Request, res: &mut Response) {
    let request = req.read_from_json::<HostRequest>().await.unwrap();

    if &request.command == "host" {
        let game = Game::from_url(&format!(
            "https://play.kahoot.it/rest/kahoots/{}",
            request.id
        ))
        .expect("Whaaaa????");
    }
}

#[derive(Serialize, Deserialize)]
struct HostRequest {
    command: String,
    id: String,
}

#[derive(Serialize, Deserialize)]
struct SuccessResponse<'a> {
    successful: bool,
    game_id: String,
    game_name: &'a str,
    question_count: usize,
}

/*
impl<'a> From<Game> for SuccessResponse<'a> {
    fn from(game: Game) -> Self {
        Self {
            successful: bool,
            // game_id,
        }
    }
}
*/
