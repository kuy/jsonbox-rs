//! Rust wrapper for [jsonbox.io](https://jsonbox.io/).
//!
//! ## Usage
//!
//! ```ignore
//! // Declaration
//! use jsonbox::{Client, Error};
//! use serde::{Deserialize, Serialize};
//!
//! // Define struct
//! #[derive(Serialize, Deserialize)]
//! pub struct Data {
//!     pub name: String,
//!     pub message: String,
//! }
//!
//! fn main() -> Result<(), Error> {
//!     // Create client with <BOX_ID>
//!     let client = Client::new("enjoy_your_first_jsonbox_rs");
//!
//!     // Put data
//!     let data = Data {
//!         name: "kuy".into(),
//!         message: "Hello, Jsonbox!".into(),
//!     };
//!     let (record, meta) = client.create(&data)?;
//!     println!("CREATE: data={:?}, meta={:?}", record, meta);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### CREATE
//!
//! ```ignore
//! let data = Data {
//!     name: "kuy".into(),
//!     message: "Hello, Jsonbox!".into(),
//! };
//! let (record, meta) = client.create(&data)?;
//! println!("CREATE: data={:?}, meta={:?}", record, meta);
//! ```
//!
//! ### READ
//!
//! #### all (default parameters)
//!
//! ```ignore
//! let all = client.read().all::<Data>()?;
//! println!("READ: len={}, all={:?}", all.len(), all);
//! ```
//!
//! #### with specific id
//!
//! ```ignore
//! let (record, meta) = client.read().id("5d876d852a780700177c0557")?;
//! println!("READ: data={:?}, meta={:?}", record, meta);
//! ```
//!
//! #### with limit
//!
//! ```ignore
//! let few = client.read().limit(10).run::<Data>()?;
//! println!("READ: len={}, few={:?}", few.len(), few);
//! ```
//!
//! #### with skip
//!
//! ```ignore
//! let rest = client.read().skip(5).run::<Data>()?;
//! println!("READ: len={}, rest={:?}", rest.len(), rest);
//! ```
//!
//! #### with order (asc/desc)
//!
//! ```ignore
//! let asc = client.read().order_by("name").run::<Data>()?;
//! println!("READ: len={}, asc={:?}", asc.len(), asc);
//!
//! let desc = client.read().order_by("count").desc().run::<Data>()?;
//! println!("READ: len={}, desc={:?}", desc.len(), desc);
//! ```
//!
//! #### with filter
//!
//! ```ignore
//! let filtered = client
//!     .read()
//!     .filter_by("name:{}", "Json Box")
//!     .run::<Data>()?;
//! println!("READ: len={}, filtered={:?}", filtered.len(), filtered);
//! ```
//!
//! See [baisc example](https://github.com/kuy/jsonbox-rs/blob/master/examples/basic.rs) or [official documentation](https://github.com/vasanthv/jsonbox#filtering) for more about filters.
//!
//! ### UPDATE
//!
//! ```ignore
//! let data = Data::new("kuy", "Hello, Jsonbox!");
//! client.update("5d876d852a780700177c0557", &data)?;
//! println!("UPDATE: OK");
//! ```
//!
//! ### DELETE
//!
//! ```ignore
//! client.delete("5d876d852a780700177c0557")?;
//! println!("DELETE: OK");
//! ```

#[cfg(test)]
extern crate matches;

mod client;
mod error;
mod url;

pub use crate::client::query_builder::QueryBuilder;
pub use crate::client::Client;
pub use crate::error::{Error, Result};
