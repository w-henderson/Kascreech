mod quiz;
mod types;

use std::{sync::Mutex, time::SystemTime};

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};

use quiz::Games;
use types::{GUIDRequest, GameIdRequest, Guess};

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

async fn generate_game(
    request: web::Json<GUIDRequest>,
    games: web::Data<Mutex<Games>>,
) -> HttpResponse {
    let mut games = games.lock().unwrap();

    match games
        .games
        .iter_mut()
        .find(|a| a.game_id() == request.game_id)
    {
        Some(game) => {
            if SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                < *game.chungus().game_start_time()
            {
                game.add_player((request.uuid.clone(), request.username.clone()));
                HttpResponse::Ok().json(&game.as_setup_game())
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        None => HttpResponse::NotFound().finish(),
    }
}

async fn chungus(games: web::Data<Mutex<Games>>) -> HttpResponse {
    let mut games = games.lock().unwrap();
    let rdr = std::fs::File::open("quizzes/topolocheese.json").unwrap();
    let questions = serde_json::from_reader(rdr).unwrap();
    games.generate_new_game(questions);
    HttpResponse::Ok().json(&games.last().unwrap().chungus())
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
    let games = web::Data::new(Mutex::new(Games::default()));

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(games.clone())
            .route("/leaderboard", web::post().to(leaderboard))
            .route("/generateGame", web::post().to(generate_game))
            .route("/makeGuess", web::post().to(handle_guess))
            .route("/chungusGameInfo", web::post().to(chungus))
            .service(actix_files::Files::new("/", "../web").index_file("index.html"))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
