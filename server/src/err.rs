use serde::Serialize;

#[macro_export]
macro_rules! send_error {
    ($w: ident, $error: expr) => {{
        log::error!("{:?}", $error);

        $w.send(Message::Text(
            serde_json::to_string(&crate::err::FailResponse::new(&$error)).unwrap(),
        ))
        .await
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
    GameNotFound,
    NameAlreadyExists,
    NotReceivingAnswers,
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
