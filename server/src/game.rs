use std::{sync::Arc, vec::IntoIter};

use serde::{Deserialize, Serialize};

use tokio::sync::mpsc::Sender;

use crate::{
    err::{FailResponse, KascreechError, KascreechResult},
    join::PlayerGuess,
    player::Player,
};

use ureq::{get, Error};

pub struct Game {
    pub game_code: String,
    pub questions: IntoIter<KahootQuestion>,
    pub players: Vec<Player>,
    pub player_sender: Sender<Arc<String>>,
}

impl Game {
    pub fn new(url: &str, player_sender: Sender<Arc<String>>) -> KascreechResult<Self> {
        match get(url).call() {
            Ok(res) => {
                let mut kahoot_game: KahootGame = res.into_json()?;

                // The time should be represented as s rather
                // than ms
                for question in &mut kahoot_game.questions {
                    question.time /= 1000;
                }

                Ok(Self {
                    game_code: kahoot_game.title,
                    questions: kahoot_game.questions.into_iter(),
                    players: Vec::new(),
                    player_sender,
                })
            }
            Err(err) => match err {
                Error::Status(e, _) if e == 404 => {
                    Err(FailResponse::new(KascreechError::GameNotFound, None))
                }
                _ => Err(err.into()),
            },
        }
    }
}

pub struct Senders {
    pub guess_sender: Sender<PlayerGuess>,
    pub receiving: bool,
}

impl Senders {
    pub const fn new(guess_sender: Sender<PlayerGuess>) -> Self {
        Self {
            guess_sender,
            receiving: false,
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
    pub question: String,
    pub time: usize,
    pub choices: Vec<KahootAnswer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KahootAnswer {
    #[serde(rename(serialize = "text"))]
    answer: String,
    pub correct: bool,
}
