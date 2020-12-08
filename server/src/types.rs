use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Guess {
    colour: String,
    uuid: String,
}

impl Default for Guess {
    fn default() -> Self {
        Self {
            uuid: "42417fdc-eae4-4d55-9d52-14d561ce6f6a".to_string(),
            colour: "red".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QuestionReply {
    correct: bool,
    #[serde(rename = "timeOfFinish")]
    time_of_finish: u128,
}

impl Default for QuestionReply {
    fn default() -> Self {
        Self {
            correct: false,
            time_of_finish: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_millis(0))
                .as_millis(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetupGame {
    answers: Vec<String>,
    #[serde(rename = "timePerQuestion")]
    time_per_question: u64, // this is milliseconds
    #[serde(rename = "timeShowingAnswers")]
    time_showing_answers: u64, // also milliseconds
    #[serde(rename = "gameStartTime")]
    game_start_time: u128, // timestamp in milliseconds of when the game starts so they're all 100% in sync, should be at least 30 seconds after request is made
}

impl SetupGame {
    pub fn new(
        answers: Vec<String>,
        time_per_question: Option<u64>,
        time_showing_answers: Option<u64>,
    ) -> Self {
        Self {
            answers,
            time_per_question: time_per_question.unwrap_or(20000),
            time_showing_answers: time_showing_answers.unwrap_or(5000),
            game_start_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_millis(0))
                .as_millis(),
        }
    }
}
