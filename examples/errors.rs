extern crate jsonbox;

use jsonbox::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub num: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Empty;

fn main() {
    let client = Client::new("kuy_________________");

    let data = Data { num: 42 };
    match client.update("xxx", &data) {
        Err(Error::General { code, message }) => {
            println!("UPDATE: code={}, message={}", code, message)
        }
        _ => println!("Failed: UPDATE: No expected errors"),
    }

    match client.create(&Empty) {
        Err(Error::General { code, message }) => {
            println!("CREATE: code={}, message={}", code, message)
        }
        _ => println!("Failed: CREATE: No expected errors"),
    }
}
