//! # API Documentation
//!
//! ## `/api/v1/import`
//! Takes a JSON object with the `id` field and imports it from Kahoot into the database. Returns the ID of the imported game in the `gameId` field.
//!
//! ## `/api/v1/featured`
//! Takes no arguments and returns featured games as an array.
//!
//! ## `/api/v1/recent`
//! Takes no arguments and lists the 10 most recent games. Returns an array of game objects.
//!
//! ## `/api/v1/search`
//! Takes a JSON object with the `query` field and searches for games with that query. Returns an array of game objects.

mod featured;
mod import;
mod not_once_cell;
mod recent;
mod search;

pub use featured::featured;
pub use import::import;
pub use recent::recent;
pub use search::search;

const COUNT: usize = 10;
