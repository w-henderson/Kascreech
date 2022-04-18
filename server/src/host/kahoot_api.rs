use crate::database::{DatabaseGame, DatabaseQuestion};
use crate::err::{FailResponse, KascreechError};
use crate::host::not_once_cell::NotOnceCell;
use crate::AppState;

use humphrey::monitor::event::{Event, EventType};
use humphrey::Client;

use humphrey_ws::{AsyncStream, Message};

use humphrey_json::prelude::*;
use humphrey_json::Value;

use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct KahootGame {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub author: String,
    pub image: Option<String>,
    pub questions: Vec<DatabaseQuestion>,
}

json_map! {
    KahootGame,
    uuid => "uuid",
    title => "title",
    description => "description",
    author => "creator_username",
    image => "cover",
    questions => "questions"
}

static CLIENT: NotOnceCell<Mutex<Client>> = NotOnceCell::new();

pub fn import(
    stream: &mut AsyncStream,
    json: Value,
    state: Arc<AppState>,
) -> Result<(), FailResponse> {
    let kahoot_id = json
        .get("id")
        .ok_or_else(FailResponse::none_option)?
        .as_str()
        .ok_or_else(FailResponse::none_option)?;

    let kahoot = get_kahoot(kahoot_id)
        .map_err(|e| FailResponse::new(KascreechError::KahootGameNotFound, Some(e.to_string())))?;

    let game = kahoot.load(stream.peer_addr());
    let id = game.id.clone();

    let response = json!({
        "success": true,
        "gameId": id
    });

    stream.send(Message::new(response.serialize()));

    let log = state.event_tx.lock().unwrap();
    log.send(
        Event::new(EventType::RequestServedSuccess)
            .with_peer(stream.peer_addr())
            .with_info(format!("Kascreech: game imported with ID {}", id)),
    )
    .ok();

    Ok(())
}

fn get_kahoot(id: &str) -> Result<DatabaseGame, Box<dyn Error>> {
    if id.is_empty() {
        return Err("No Kahoot ID provided".into());
    }

    let mut client = CLIENT.get_or_init(|| Mutex::new(Client::new())).lock()?;
    let response = client
        .get(format!("https://play.kahoot.it/rest/kahoots/{}", id))?
        .send()?;
    let game: KahootGame = humphrey_json::from_str(response.text().ok_or("Invalid response")?)?;

    Ok(DatabaseGame {
        id: format!("kahoot-{}", game.uuid),
        name: game.title,
        description: game.description,
        author: game.author,
        image: game.image,
        questions: game.questions,
        plays: 0,
        kahoot: true,
    })
}
