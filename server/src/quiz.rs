use std::{
    cmp::Ordering,
    iter::repeat_with,
    time::{Duration, SystemTime},
};

use crate::types::{Guess, SetupGame};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Games {
    pub games: Vec<Game>,
}

impl Games {
    pub fn generate_new_game(&mut self, questions: Questions) {
        let game_id: String = repeat_with(fastrand::alphanumeric).take(6).collect();
        let new_game = Game::new(game_id, questions, None, None, None);
        self.games.push(new_game);
    }
    pub fn last(&self) -> Option<&Game> {
        self.games.last()
    }
}

#[derive(Debug)]
pub struct Game {
    game_id: String,
    pub players: Players,
    chungus: Chungus,
}

impl Game {
    pub fn new(
        game_id: String,
        questions: Questions,
        time_per_question: Option<u64>,
        time_showing_answers: Option<u64>,
        time_showing_leaderboard: Option<u64>,
    ) -> Self {
        let chungus = Chungus::new(
            questions,
            time_per_question,
            time_showing_answers,
            time_showing_leaderboard,
            game_id.clone(),
        );
        Self {
            game_id,
            players: Players::default(),
            chungus,
        }
    }
    pub fn chungus(&self) -> &Chungus {
        &self.chungus
    }
    pub fn as_setup_game(&self) -> SetupGame {
        SetupGame::new(
            self.chungus.questions.answers(),
            Some(self.chungus.time_per_question),
            Some(self.chungus.time_showing_answers),
            Some(self.chungus.time_showing_leaderboard),
            self.chungus.game_start_time,
        )
    }
    pub fn add_score(&mut self, guess: Guess) {
        for player in self.players.players.iter_mut() {
            if player.uuid == guess.uuid {
                player.score += guess.score;
            }
        }
    }
    pub fn add_player(&mut self, uuid: String) {
        let new_player = Player::from(uuid);
        self.players.players.push(new_player);
    }
    pub fn sort(&mut self) {
        self.players.players.sort();
    }
    pub fn game_id(&self) -> &str {
        &self.game_id
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default, Ord, PartialOrd)]
pub struct Players {
    players: Vec<Player>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
struct Player {
    uuid: String,
    score: u32,
}

impl From<String> for Player {
    fn from(uuid: String) -> Self {
        Self { uuid, score: 0 }
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

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Questions {
    questions: Vec<QAndA>,
}

impl Questions {
    fn answers(&self) -> Vec<Vec<usize>> {
        self.questions.iter().map(|a| a.correct.clone()).collect()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QAndA {
    // The given input question
    question: String,
    // A list of possible answers
    responses: [String; 4],
    // The true answer's index in 'responses'
    correct: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Chungus {
    #[serde(rename = "bigChungus")]
    big_chungus: bool,
    questions: Questions,
    #[serde(rename = "timePerQuestion")]
    time_per_question: u64, // this is milliseconds
    #[serde(rename = "timeShowingAnswers")]
    time_showing_answers: u64, // also milliseconds
    #[serde(rename = "timeShowingLeaderboard")]
    time_showing_leaderboard: u64,
    #[serde(rename = "gameStartTime")]
    game_start_time: u128, // timestamp in milliseconds of when the game starts
    #[serde(rename = "gameId")]
    game_id: String,
}

impl Chungus {
    fn new(
        questions: Questions,
        time_per_question: Option<u64>,
        time_showing_answers: Option<u64>,
        time_showing_leaderboard: Option<u64>,
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
}
