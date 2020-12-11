use std::{
    cmp::Ordering,
    iter::repeat_with,
    time::{Duration, SystemTime},
};

use crate::types::{Guess, SetupGame};

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Games {
    pub games: Vec<Game>,
}

impl Games {
    /// Adds a game with specified quesions
    pub fn generate_new_game(&mut self, questions: Vec<QAndA>) {
        let game_id: String = repeat_with(|| fastrand::digit(10)).take(6).collect();
        let new_game = Game::new(game_id, questions, None, None, None);
        self.games.push(new_game);
    }
    /// Checks over `self`, looking for games that're
    /// alive for longer than their lifetime, removing
    /// them if they are
    pub fn check(&mut self) {
        self.games.drain_filter(|game| {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                > game.to_remove
        });
    }
}

#[derive(Serialize, Default)]
pub struct Players {
    pub players: Vec<Player>,
}

pub struct Game {
    pub players: Players,
    chungus: Chungus,
    // The timestamp of when the game should die
    to_remove: u128,
}

impl Game {
    pub fn new(
        game_id: String,
        questions: Vec<QAndA>,
        time_per_question: Option<u128>,
        time_showing_answers: Option<u128>,
        time_showing_leaderboard: Option<u128>,
    ) -> Self {
        let chungus = Chungus::new(
            questions,
            time_per_question,
            time_showing_answers,
            time_showing_leaderboard,
            game_id,
        );
        let len = chungus.questions.len() as u128;
        let to_remove = chungus.game_start_time
            + (chungus.time_per_question
                + chungus.time_showing_answers
                + chungus.time_showing_leaderboard)
                * len;
        Self {
            players: Players::default(),
            chungus,
            to_remove,
        }
    }
    pub fn chungus(&self) -> &Chungus {
        &self.chungus
    }
    pub fn as_setup_game(&self) -> SetupGame {
        SetupGame::new(
            self.chungus.answers(),
            Some(self.chungus.time_per_question),
            Some(self.chungus.time_showing_answers),
            Some(self.chungus.time_showing_leaderboard),
            self.chungus.game_start_time,
        )
    }
    pub fn add_score(&mut self, guess: Guess) {
        let uuid = guess.uuid;
        let score = guess.score;
        match self.players.players.iter_mut().find(|p| p.uuid == uuid) {
            Some(p) => p.score += score,
            None => {}
        }
    }
    pub fn add_player(&mut self, id: (String, String)) {
        let new_player = Player::from(id);
        self.players.players.push(new_player);
    }
    pub fn sort(&mut self) {
        self.players.players.sort();
    }
    pub fn game_id(&self) -> &str {
        &self.chungus.game_id
    }
}

#[derive(Serialize, PartialEq, Eq)]
pub struct Player {
    uuid: String,
    username: String,
    score: u32,
}

impl From<(String, String)> for Player {
    fn from(id: (String, String)) -> Self {
        Self {
            uuid: id.0,
            username: id.1,
            score: 0,
        }
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

#[derive(Serialize, Deserialize)]
pub struct QAndA {
    // The given input question
    question: String,
    // A list of possible answers
    responses: [String; 4],
    // The true answer's index in 'responses'
    correct: Vec<usize>,
}

#[derive(Serialize)]
pub struct Chungus {
    #[serde(rename = "bigChungus")]
    big_chungus: bool,
    questions: Vec<QAndA>,
    #[serde(rename = "timePerQuestion")]
    time_per_question: u128, // this is milliseconds
    #[serde(rename = "timeShowingAnswers")]
    time_showing_answers: u128, // also milliseconds
    #[serde(rename = "timeShowingLeaderboard")]
    time_showing_leaderboard: u128,
    #[serde(rename = "gameStartTime")]
    game_start_time: u128, // timestamp in milliseconds of when the game starts
    #[serde(rename = "gameId")]
    game_id: String,
}

impl Chungus {
    fn new(
        questions: Vec<QAndA>,
        time_per_question: Option<u128>,
        time_showing_answers: Option<u128>,
        time_showing_leaderboard: Option<u128>,
        game_id: String,
    ) -> Self {
        Self {
            big_chungus: true,
            questions,
            time_per_question: time_per_question.unwrap_or(20000),
            time_showing_answers: time_showing_answers.unwrap_or(5000),
            time_showing_leaderboard: time_showing_leaderboard.unwrap_or(5000),
            game_start_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_millis(0))
                .as_millis()
                + 30_000,
            game_id,
        }
    }
    pub const fn game_start_time(&self) -> &u128 {
        &self.game_start_time
    }
    fn answers(&self) -> Vec<&Vec<usize>> {
        self.questions.iter().map(|a| &a.correct).collect()
    }
}
