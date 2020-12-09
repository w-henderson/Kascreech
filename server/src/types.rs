use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Guess {
    #[serde(rename = "gameId")]
    pub game_id: String,
    pub uuid: String,
    pub score: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetupGame {
    answers: Vec<Vec<usize>>,
    #[serde(rename = "timePerQuestion")]
    time_per_question: u64, // this is milliseconds
    #[serde(rename = "timeShowingAnswers")]
    time_showing_answers: u64, // also milliseconds
    #[serde(rename = "timeShowingLeaderboard ")]
    time_showing_leaderboard: u64,
    #[serde(rename = "gameStartTime")]
    game_start_time: u128, // timestamp in milliseconds of when the game starts so they're all 100% in sync, should be at least 30 seconds after request is made
}

impl SetupGame {
    pub fn new(
        answers: Vec<Vec<usize>>,
        time_per_question: Option<u64>,
        time_showing_answers: Option<u64>,
        time_showing_leaderboard: Option<u64>,
    ) -> Self {
        Self {
            answers,
            time_per_question: time_per_question.unwrap_or(20000),
            time_showing_answers: time_showing_answers.unwrap_or(5000),
            time_showing_leaderboard: time_showing_leaderboard.unwrap_or(5000),
            game_start_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_millis(0))
                .as_millis()
                + 30_000,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameIdRequest {
    #[serde(rename = "gameId")]
    pub game_id: String,
}
