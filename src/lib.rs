#[cfg(test)]
extern crate matches;

pub mod client;
mod url;

pub use crate::client::{Client, Error, Result};
