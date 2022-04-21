//! # API Documentation
//!
//! ## `/api/v1/import`
//! Takes a JSON object with the `id` field and imports it from Kahoot into the database. Returns the ID of the imported game in the `gameId` field.
//!
//! ## `/api/v1/featured`
//! Takes no arguments and returns featured games as an array.
//!
//! ## `/api/v1/list`
//! Takes a JSON object with the `offset` field and lists 10 games starting at that offset. Returns an array of game objects.
//!
//! ## `/api/v1/search`
//! Takes a JSON object with the `query` field and searches for games with that query. Returns an array of game objects.

mod featured;
mod import;
mod list;
mod not_once_cell;
mod search;

pub use featured::featured;
pub use import::import;
pub use list::list;
pub use search::search;

const COUNT: usize = 10;
