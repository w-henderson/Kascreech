use futures::StreamExt;
use serde::{Deserialize, Serialize};

use salvo::{
    async_trait, extra::ws::WsHandler, fn_handler, prelude::HttpError, Request, Response, Writer,
};

use crate::game::Game;

#[fn_handler]
pub async fn new_game_handle(req: &mut Request, res: &mut Response) -> Result<(), HttpError> {
    let fut = WsHandler::new().handle(req, res)?;
    let fut = async move {
        if let Some(ws) = fut.await {
            let (sink, mut stream) = ws.split();

            if let Some(Ok(msg)) = stream.next().await {
                println!("{:?}", msg.to_str().unwrap());
            }

            // let req = stream.next().await;
        }
    };
    tokio::task::spawn(fut);
    /*let request = req.read_from_json::<HostRequest>().await.unwrap();

    if &request.command == "host" {
        if let Ok(game) = Game::from_url(&format!(
            "https://play.kahoot.it/rest/kahoots/{}",
            request.id
        )) {
            let game_id = repeat_with(|| fastrand::digit(10))
                .take(6)
                .collect::<String>();

            res.render_json(&game.make_response(&game_id));
        }
    }

    Ok(())*/
}

#[derive(Deserialize)]
struct HostRequest {
    command: String,
    id: String,
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
