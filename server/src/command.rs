use serde::{Deserialize, Serialize};

/// A generic command
#[derive(Debug, Serialize, Deserialize)]
pub struct Command<'a> {
    pub command: &'a str,
}
