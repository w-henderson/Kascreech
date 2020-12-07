mod types;
use types::{Guess, QuestionReply};

use actix_web::{dev::HttpResponseBuilder, http::StatusCode, web, App, HttpResponse, HttpServer};

async fn guess(evt: web::Json<Guess>) -> HttpResponse {
    println!("{:?}", evt);

    HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(QuestionReply::default())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/postResponse")
                .route(web::post().to(guess))
                .default_service(web::route().to(HttpResponse::NotFound)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
