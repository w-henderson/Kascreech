use serde::Serialize;

pub type KascreechResult<T> = std::result::Result<T, FailResponse>;

/// A macro to log an error, and write it
/// to a given sink
#[macro_export]
macro_rules! send_error {
    ($w: ident, $error: expr) => {{
        log::error!("{}", $error);

        $w.send(Message::Text(serde_json::to_string(&$error).unwrap()))
            .await
            .unwrap();

        return Err($error);
    }};
}

#[derive(Serialize)]
pub struct FailResponse {
    success: bool,
    #[serde(rename = "errorType")]
    error_type: KascreechError,
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
}

impl std::fmt::Display for FailResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(error_message) = &self.error_message {
            write!(f, "{} {}", self.error_type, error_message)
        } else {
            write!(f, "{}", self.error_type)
        }
    }
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

#[derive(Serialize, Debug)]
pub enum KascreechError {
    IoError,
    UreqError,
    SerdeError,
    TungsteniteError,
    FailedRead,
    GameNotFound,
    NameAlreadyExists,
    UnrecognisedCommand,
}

impl std::fmt::Display for KascreechError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KascreechError::IoError => write!(f, "io error"),
            KascreechError::UreqError => write!(f, "ureq error"),
            KascreechError::SerdeError => write!(f, "serde error"),
            KascreechError::TungsteniteError => write!(f, "tungstenite error"),
            KascreechError::FailedRead => write!(f, "failed to read from a stream"),
            KascreechError::GameNotFound => write!(f, "a requested game_id didn't exist"),
            KascreechError::NameAlreadyExists => write!(f, "a requested name already existed"),
            KascreechError::UnrecognisedCommand => write!(f, "unknown command"),
        }
    }
}

impl From<std::io::Error> for FailResponse {
    fn from(io_error: std::io::Error) -> Self {
        Self::new(KascreechError::IoError, Some(io_error.to_string()))
    }
}

impl From<ureq::Error> for FailResponse {
    fn from(ureq_error: ureq::Error) -> Self {
        Self::new(KascreechError::UreqError, Some(ureq_error.to_string()))
    }
}

impl From<serde_json::Error> for FailResponse {
    fn from(serde_error: serde_json::Error) -> Self {
        Self::new(KascreechError::SerdeError, Some(serde_error.to_string()))
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for FailResponse {
    fn from(tungstenite_error: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::new(
            KascreechError::TungsteniteError,
            Some(tungstenite_error.to_string()),
        )
    }
}
