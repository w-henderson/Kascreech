use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Games(Vec<Game>);

#[derive(Serialize, Deserialize)]
pub struct Game {
    title: String,
    questions: Vec<Question>,
}

impl Game {
    pub fn from_url(path: &str) -> Result<Self, std::io::Error> {
        ureq::get(path).call().unwrap().into_json::<Self>()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    question: String,
    time: usize,
    choices: Vec<Answer>,
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    answer: String,
    correct: bool,
}
