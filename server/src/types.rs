use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "colour")]
pub enum Colour {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
}

impl Default for Colour {
    fn default() -> Self {
        Self::Red
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Guess {
    colour: Colour,
    #[serde(rename = "USER_ID")]
    uuid: String,
}

impl Default for Guess {
    fn default() -> Self {
        Self {
            uuid: "42417fdc-eae4-4d55-9d52-14d561ce6f6a".to_string(),
            ..Default::default()
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
