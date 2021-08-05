use std::{sync::Arc, vec::IntoIter};

use serde::{Deserialize, Serialize};

use tokio::sync::mpsc::Sender;

use crate::{err::KascreechError, join::PlayerGuess, player::Player};

use ureq::{get, Error};

pub struct Game {
    pub game_code: String,
    pub questions: IntoIter<KahootQuestion>,
    pub players: Vec<Player>,
    pub player_sender: Sender<Arc<String>>,
    pub question_sender: Sender<PlayerGuess>,
    /// Whether the game is currently receiving more questions
    pub receiving: bool,
}

impl Game {
    pub fn new(
        url: &str,
        player_sender: Sender<Arc<String>>,
        question_sender: Sender<PlayerGuess>,
    ) -> Result<Self, KascreechError> {
        match get(url).call() {
            Ok(res) => {
                let kahoot_game: KahootGame = res.into_json()?;

                Ok(Self {
                    game_code: kahoot_game.title,
                    questions: kahoot_game.questions.into_iter(),
                    players: Vec::new(),
                    player_sender,
                    question_sender,
                    receiving: false,
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
