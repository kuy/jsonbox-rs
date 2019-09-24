#[cfg(test)]
extern crate matches;

pub mod client;
mod error;
mod query_builder;
mod url;

pub use crate::error::{Error, Result};
pub use crate::client::Client;
