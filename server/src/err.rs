use serde::Serialize;

pub type KascreechResult<T> = std::result::Result<T, KascreechError>;

/// A macro to log an error, and write it
/// to a given sink
#[macro_export]
macro_rules! send_error {
    ($w: ident, $error: expr) => {{
        log::error!("{:?}", $error);

        $w.send(Message::Text(
            serde_json::to_string(&crate::err::FailResponse::new(&$error)).unwrap(),
        ))
        .await
        .unwrap();

        return Err($error);
    }};
}

#[derive(Serialize)]
pub struct FailResponse<'a> {
    success: bool,
    message: &'a KascreechError,
}

impl<'a> FailResponse<'a> {
    pub const fn new(message: &'a KascreechError) -> Self {
        Self {
            success: false,
            message,
        }
    }
}

#[derive(Serialize, Debug)]
pub enum KascreechError {
    IoErr(String),
    UreqError(String),
    SerdeError(String),
    TungsteniteError(String),
    FailedRead,
    GameNotFound,
    NameAlreadyExists,
    UnrecognisedCommand(String),
}

impl std::fmt::Display for KascreechError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KascreechError::IoErr(s) => write!(f, "io error {}", s),
            KascreechError::UreqError(s) => write!(f, "ureq error {}", s),
            KascreechError::SerdeError(s) => write!(f, "serde error {}", s),
            KascreechError::TungsteniteError(s) => write!(f, "tungstenite error {}", s),
            KascreechError::FailedRead => write!(f, "failed to read from a stream"),
            KascreechError::GameNotFound => write!(f, "a requested game_id didn't exist"),
            KascreechError::NameAlreadyExists => write!(f, "a requested name already existed"),
            KascreechError::UnrecognisedCommand(s) => write!(f, "unknown command {}", s),
        }
    }
}

impl From<std::io::Error> for KascreechError {
    fn from(io_err: std::io::Error) -> Self {
        Self::IoErr(io_err.to_string())
    }
}

impl From<ureq::Error> for KascreechError {
    fn from(ureq_err: ureq::Error) -> Self {
        Self::UreqError(ureq_err.to_string())
    }
}

impl From<serde_json::Error> for KascreechError {
    fn from(serde_error: serde_json::Error) -> Self {
        Self::SerdeError(serde_error.to_string())
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for KascreechError {
    fn from(tungstenite_error: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::TungsteniteError(tungstenite_error.to_string())
    }
}
