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
    KahootGameNotFound,
    UsernameAlreadyExists,
    InvalidCommand,
    UnknownError,
}

impl FailResponse {
    pub fn new(error_type: KascreechError, error_message: Option<String>) -> Self {
        Self {
            success: false,
            error_type,
            error_message,
        }
    }

    pub fn none_option() -> Self {
        Self {
            success: false,
            error_type: KascreechError::FailedRead,
            error_message: Some("A parameter was missing".into()),
        }
    }
}

impl From<ParseError> for FailResponse {
    fn from(error: ParseError) -> Self {
        Self::new(KascreechError::FailedRead, Some(error.to_string()))
    }
}

impl IntoJson for KascreechError {
    fn to_json(&self) -> Value {
        match self {
            KascreechError::FailedRead => json!("FailedRead"),
            KascreechError::GameNotFound => json!("GameNotFound"),
            KascreechError::KahootGameNotFound => json!("KahootGameNotFound"),
            KascreechError::UsernameAlreadyExists => json!("UsernameAlreadyExists"),
            KascreechError::InvalidCommand => json!("InvalidCommand"),
            KascreechError::UnknownError => json!("UnknownError"),
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
