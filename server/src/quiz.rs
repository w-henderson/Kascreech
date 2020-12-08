use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Game {
    pub players: Vec<(String, u32)>,
}

// This struct should be serialized and deserialized to the .json files (Could use other files for better efficiency)
#[derive(Deserialize, Serialize, Debug)]
pub struct Questions {
    questions: Vec<QAndA>,
}

// A struct showing a single round
#[derive(Deserialize, Serialize, Debug)]
pub struct QAndA {
    // The given input question
    question: String,
    // A list of possible answers
    responses: Vec<String>,
    // The true answer's index in 'responses'
    correct: usize,
}

impl QAndA {
    pub fn new(question: String, responses: Vec<String>, correct: usize) -> Self {
        Self {
            question,
            responses,
            correct,
        }
    }
    /*pub fn is_right(&self, guess: String) -> bool {
        self.responses[self.correct] == guess
    }
    pub fn get_answer(&self) -> &str {
        &self.responses[self.correct]
    }*/
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chungus {
    #[serde(rename = "bigChungus")]
    big_chungus: bool,
    questions: Vec<QAndA>,
    #[serde(rename = "timePerQuestion")]
    time_per_question: u64, // this is milliseconds
    #[serde(rename = "timeShowingAnswers")]
    time_showing_answers: u64, // also milliseconds
    #[serde(rename = "gameStartTime")]
    game_start_time: u128, // timestamp in milliseconds of when the game starts
    #[serde(rename = "gameId")]
    game_id: String,
}

impl Chungus {
    pub fn new(
        questions: Vec<QAndA>,
        time_per_question: Option<u64>,
        time_showing_answers: Option<u64>,
    ) -> Self {
        Self {
            big_chungus: true,
            questions,
            time_per_question: time_per_question.unwrap_or(20000),
            time_showing_answers: time_showing_answers.unwrap_or(5000),
            game_start_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_millis(0))
                .as_millis(),
            game_id: "Chungus".to_string(),
        }
    }
}
