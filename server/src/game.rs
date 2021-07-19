use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use tokio::sync::mpsc::Sender;

use crate::{err::KascreechError, player::Player};

use ureq::{get, Error};

pub struct Game {
    pub title: String,
    pub questions: Vec<KahootQuestion>,
    pub players: HashMap<String, Player>,
    pub host_sender: Sender<String>,
}

impl Game {
    pub fn new(url: &str, host_sender: Sender<String>) -> Result<Self, KascreechError> {
        match get(url).call() {
            Ok(res) => {
                let kahoot_game: KahootGame = res.into_json()?;

                Ok(Self {
                    title: kahoot_game.title,
                    questions: kahoot_game.questions,
                    players: HashMap::new(),
                    host_sender,
                })
            }
            Err(err) => match err {
                Error::Status(e, _) if e == 404 => Err(KascreechError::GameNotFound),
                _ => Err(err.into()),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct KahootGame {
    title: String,
    questions: Vec<KahootQuestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KahootQuestion {
    question: String,
    time: usize,
    choices: Vec<KahootAnswer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KahootAnswer {
    answer: String,
    correct: bool,
}
