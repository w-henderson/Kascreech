use crate::{err::FailResponse, AppState};

use humphrey_ws::AsyncStream;

use humphrey_json::Value;

use std::sync::Arc;

pub fn join(
    stream: &mut AsyncStream,
    json: Value,
    state: Arc<AppState>,
) -> Result<(), FailResponse> {
    Ok(())
}
