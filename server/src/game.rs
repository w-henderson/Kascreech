use serde::{Deserialize, Serialize};

use crate::err::KascreechError;

use ureq::get;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub title: String,
    pub questions: Vec<Question>,
}

impl Game {
    pub fn from_url(path: &str) -> Result<Self, KascreechError> {
        get(path).call()?.into_json().map_err(Into::into)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    question: String,
    time: usize,
    choices: Vec<Answer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    answer: String,
    correct: bool,
}
