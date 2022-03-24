use humphrey::Client;

use humphrey_json::prelude::*;

use std::error::Error;
use std::lazy::SyncOnceCell;
use std::sync::Mutex;

use crate::types::{Answer, Question};

static CLIENT: SyncOnceCell<Mutex<Client>> = SyncOnceCell::new();

pub struct KahootGame {
    pub title: String,
    pub questions: Vec<KahootQuestion>,
}

pub struct KahootQuestion {
    pub question: String,
    pub time: usize,
    pub choices: Vec<KahootAnswer>,
}

pub struct KahootAnswer {
    pub answer: String,
    pub correct: bool,
}

json_map! {
    KahootGame,
    title => "title",
    questions => "questions"
}

json_map! {
    KahootQuestion,
    question => "question",
    time => "time",
    choices => "choices"
}

json_map! {
    KahootAnswer,
    answer => "answer",
    correct => "correct"
}

pub fn get_kahoot(id: &str) -> Result<KahootGame, Box<dyn Error>> {
    let mut client = CLIENT.get_or_init(|| Mutex::new(Client::new())).lock()?;
    let response = client
        .get(format!("https://play.kahoot.it/rest/kahoots/{}", id))?
        .send()?;
    let game: KahootGame = humphrey_json::from_str(response.text().ok_or("Invalid response")?)?;

    Ok(game)
}

pub fn generate_id() -> String {
    let mut buf = [0u8; 4];
    getrandom::getrandom(&mut buf).expect("Failed to generate random ID");

    let id = u32::from_be_bytes(buf) % 1_000_000;

    format!("{:06}", id)
}

pub fn kahoot_questions_to_normal_questions(
    kahoot_questions: Vec<KahootQuestion>,
) -> Vec<Question> {
    kahoot_questions
        .into_iter()
        .map(|q| Question {
            question: q.question,
            duration: q.time / 1000,
            answers: q
                .choices
                .into_iter()
                .map(|a| Answer {
                    text: a.answer,
                    correct: a.correct,
                })
                .collect(),
        })
        .collect()
}
