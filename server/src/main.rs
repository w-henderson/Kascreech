#![warn(clippy::nursery, clippy::pedantic)]

mod game;

mod new_game;

use new_game::new_game_handle;

use salvo::{async_trait, fn_handler, Response, Router, Server};

#[fn_handler]
async fn hello_world(res: &mut Response) {
    res.render_plain_text("Hello World");
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .handle(hello_world)
        .push(Router::new().path("newGame").handle(new_game_handle));
    Server::new(router).bind(([0, 0, 0, 0], 7878)).await;
}
