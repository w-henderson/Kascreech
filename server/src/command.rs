use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Command<'a> {
    pub command: &'a str,
}
