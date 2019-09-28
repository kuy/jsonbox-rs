extern crate jsonbox;

use jsonbox::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    num: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Empty;

fn main() {
    let client = Client::new("kuy_00000000000000000000");

    let data = Data { num: 42 };
    match client.update("11111111111111111111", &data) {
        Err(Error::General { code, message }) => {
            println!("UPDATE: code={}, message={}", code, message)
        }
        _ => println!("Failed: UPDATE: No errors"),
    }

    match client.delete("11111111111111111111") {
        Err(Error::General { code, message }) => {
            println!("CREATE: code={}, message={}", code, message)
        }
        _ => println!("Failed: DELETE: No errors"),
    }
}
