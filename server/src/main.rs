mod types;

use types::{Guess, QuestionReply};

use actix_cors::Cors;
use actix_web::{dev::HttpResponseBuilder, http::StatusCode, web, App, HttpResponse, HttpServer};

async fn guess(evt: web::Json<Guess>) -> HttpResponse {
    println!("{:?}", evt);

    let to_send = QuestionReply::default();

    HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(to_send)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new().wrap(cors).service(
            web::resource("/postResponse")
                .route(web::post().to(guess))
                .default_service(web::route().to(HttpResponse::NotFound)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
