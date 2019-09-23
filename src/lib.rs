#[cfg(test)]
extern crate matches;

pub mod client;
mod query_builder;
mod url;

pub use crate::client::{Client, Error, Result};
