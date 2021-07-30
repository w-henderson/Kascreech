use serde::Serialize;

use tokio_tungstenite::tungstenite::Message;

use log::error;

use futures::SinkExt;

pub async fn send_error<W: SinkExt<Message> + Unpin + Send>(
    w: &mut W,
    error: &KascreechError,
) -> Result<(), W::Error> {
    error!("{:?}", error);

    let message = Message::Text(serde_json::to_string(&FailResponse::new(error)).unwrap());

    w.send(message).await?;

    Ok(())
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
    GameNotFound,
    NameAlreadyExists,
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
