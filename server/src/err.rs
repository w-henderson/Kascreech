use serde::Serialize;

#[derive(Serialize)]
pub struct FailResponse {
    success: bool,
    message: KascreechError,
}

#[derive(Serialize, Debug)]
pub enum KascreechError {
    IoErr(String),
    UreqError(String),
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
