use crate::types::{Answer, Game, GamePhase, Question};

use humphrey_json::prelude::*;

use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct DatabaseGame {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub image: Option<String>,
    pub questions: Vec<DatabaseQuestion>,
    pub plays: usize,
    pub kahoot: bool,
}

#[derive(Clone, Debug)]
pub struct DatabaseQuestion {
    pub question: String,
    pub time: usize,
    pub choices: Vec<DatabaseAnswer>,
}

#[derive(Clone, Debug)]
pub struct DatabaseAnswer {
    pub answer: String,
    pub correct: bool,
}

json_map! {
    DatabaseGame,
    id => "id",
    name => "name",
    description => "description",
    author => "author",
    image => "image",
    questions => "questions",
    plays => "plays",
    kahoot => "kahoot"
}

json_map! {
    DatabaseQuestion,
    question => "question",
    time => "time",
    choices => "choices"
}

json_map! {
    DatabaseAnswer,
    answer => "answer",
    correct => "correct"
}

impl DatabaseGame {
    pub fn load(&self, addr: SocketAddr) -> Game {
        let id = generate_id();

        let questions = self
            .questions
            .clone()
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
            .collect::<Vec<_>>()
            .into_iter();

        Game {
            id,
            questions,
            phase: GamePhase::Lobby,
            players: HashMap::new(),
            host: addr,
            correct_answers: Vec::new(),
            question_start_time: 0,
        }
    }
}

fn generate_id() -> String {
    let mut buf = [0u8; 4];
    getrandom::getrandom(&mut buf).expect("Failed to generate random ID");

    let id = u32::from_be_bytes(buf) % 1_000_000;

    format!("{:06}", id)
}
