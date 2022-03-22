use humphrey_json::error::ParseError;
use humphrey_json::prelude::*;
use humphrey_json::Value;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::vec::IntoIter;

#[derive(Clone)]
pub enum ClientStatus {
    Loading,
    Playing(String),
    Hosting(String),
}

pub struct Game {
    pub id: String,
    pub questions: IntoIter<Question>,
    pub phase: GamePhase,
    pub players: HashMap<SocketAddr, Player>,
    pub host: SocketAddr,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    Lobby,
    Question,
    Leaderboard,
}

pub struct Player {
    pub name: String,
    pub points: usize,
    pub streak: usize,
    pub played: bool,
    pub player_round_end: Option<PlayerRoundEnd>,
}

pub struct PlayerRoundEnd {
    pub event: String,
    pub correct: bool,
    pub points_this_round: usize,
    pub points_total: usize,
    pub streak: usize,
    pub position: usize,
    pub behind: Option<String>,
}

pub struct LeaderboardMessage {
    pub leaderboard: Vec<Player>,
}

pub struct Question {
    pub question: String,
    pub duration: usize,
    pub answers: Vec<Answer>,
}

pub struct Answer {
    pub text: String,
    pub correct: bool,
}

impl IntoJson for Player {
    fn to_json(&self) -> Value {
        json!({
            "userName": (self.name.clone()),
            "points": (self.points),
            "streak": (self.streak)
        })
    }
}

impl FromJson for Player {
    fn from_json(_: &Value) -> Result<Self, ParseError> {
        Err(ParseError::UnknownError)
    }
}

json_map! {
    LeaderboardMessage,
    leaderboard => "leaderboard"
}

json_map! {
    PlayerRoundEnd,
    event => "event",
    correct => "correct",
    points_this_round => "pointsThisRound",
    points_total => "pointsTotal",
    streak => "streak",
    position => "position",
    behind => "behind"
}

json_map! {
    Question,
    question => "question",
    duration => "duration",
    answers => "answers"
}

json_map! {
    Answer,
    text => "text",
    correct => "correct"
}
