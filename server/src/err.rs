use humphrey_json::error::ParseError;
use humphrey_json::prelude::*;
use humphrey_json::Value;

pub struct FailResponse {
    pub success: bool,
    pub error_type: KascreechError,
    pub error_message: Option<String>,
}

pub enum KascreechError {
    FailedRead,
    GameNotFound,
    NameAlreadyExists,
    UnrecognisedCommand,
}

impl FailResponse {
    pub fn new(error_type: KascreechError, error_message: Option<String>) -> Self {
        Self {
            success: false,
            error_type,
            error_message,
        }
    }
}

impl IntoJson for KascreechError {
    fn to_json(&self) -> Value {
        match self {
            KascreechError::FailedRead => json!("FailedRead"),
            KascreechError::GameNotFound => json!("GameNotFound"),
            KascreechError::NameAlreadyExists => json!("NameAlreadyExists"),
            KascreechError::UnrecognisedCommand => json!("UnrecognisedCommand"),
        }
    }
}

impl FromJson for KascreechError {
    fn from_json(_: &Value) -> Result<Self, ParseError> {
        Err(ParseError::UnknownError)
    }
}

json_map! {
    FailResponse,
    success => "success",
    error_type => "errorType",
    error_message => "errorMessage"
}
