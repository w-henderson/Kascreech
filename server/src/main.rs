mod quiz;
mod types;

use quiz::{Chungus, QAndA};
use types::{Guess, QuestionReply, SetupGame};

use actix_cors::Cors;
use actix_web::{dev::HttpResponseBuilder, http::StatusCode, web, App, HttpResponse, HttpServer};

async fn guess(_evt: web::Json<Guess>) -> HttpResponse {
    HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(QuestionReply::default())
}

async fn generate_game() -> HttpResponse {
    HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(SetupGame::new(
        vec![
            "red".to_string(),
            "yellow".to_string(),
            "blue".to_string(),
            "red".to_string(),
            "green".to_string(),
            "green".to_string(),
        ],
        None,
        None,
    ))
}

async fn chungus() -> HttpResponse {
    let question = QAndA::new(
        "Which of the following is not a cheese?".to_string(),
        vec![
            "Elliot's feet".to_string(),
            "Elliot's head".to_string(),
            "Elliot's nose".to_string(),
            "Elliot".to_string(),
        ],
        2,
    );
    let chungus = Chungus::new(vec![question], None, None);
    HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(chungus)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .route("/generateGame", web::post().to(generate_game))
            .route("/postResponse", web::post().to(guess))
            .route("/chungusGameInfo", web::post().to(chungus))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
