mod quiz;
mod types;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use quiz::{Game, Games};
use types::{GameIdRequest, Guess};

async fn hello_world() -> impl Responder {
    "Hello, World!".to_string()
}

async fn handle_guess(request: web::Json<Guess>, games: web::Data<Mutex<Games>>) -> HttpResponse {
    let mut games = games.lock().unwrap();
    match games
        .games
        .iter_mut()
        .find(|a| a.game_id() == request.game_id)
    {
        Some(game) => {
            game.add_score(request.into_inner());
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}

async fn generate_game(request: web::Json<GameIdRequest>, games: web::Data<Games>) -> HttpResponse {
    match games.games.iter().find(|a| a.game_id() == request.game_id) {
        Some(game) => HttpResponse::Ok().json(&game.as_setup_game()),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn chungus(request: web::Json<GameIdRequest>, games: web::Data<Games>) -> HttpResponse {
    match games.games.iter().find(|a| a.game_id() == request.game_id) {
        Some(game) => HttpResponse::Ok().json(&game.chungus()),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn leaderboard(
    request: web::Json<GameIdRequest>,
    games: web::Data<Mutex<Games>>,
) -> HttpResponse {
    let mut games = games.lock().unwrap();
    match games
        .games
        .iter_mut()
        .find(|a| a.game_id() == request.game_id)
    {
        Some(game) => {
            game.sort();
            HttpResponse::Ok().json(&game.players)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        let games = {
            let rdr = std::fs::File::open("questions/default.json").unwrap();
            let questions = serde_json::from_reader(rdr).unwrap();
            let game = Game::new("Default".to_string(), questions, None, None, None);
            let mut games = Games::default();
            games.games.push(game);
            games
        };

        App::new()
            .data(games)
            .wrap(cors)
            .route("/test", web::get().to(hello_world))
            .route("/leaderboard", web::post().to(leaderboard))
            .route("/generateGame", web::post().to(generate_game))
            .route("/makeGuess", web::post().to(handle_guess))
            .route("/chungusGameInfo", web::post().to(chungus))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
