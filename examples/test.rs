extern crate jsonbox;

use jsonbox::client::{Client, Error};
use serde::{Deserialize, Serialize};

const BOX_ID: &str = "box_ed82aef3f93176996145";

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub name: String,
    pub message: String,
}

impl Data {
    fn new(name: &str, message: &str) -> Data {
        Data {
            name: name.to_string(),
            message: message.to_string(),
        }
    }
}

fn main() -> Result<(), Error> {
    let client = Client::new(BOX_ID);

    let data = Data::new("kuy", "Hello, Jsonbox!");
    let record: Data = client.create(&data)?;
    println!("{:?}", record);

    let records: Vec<Data> = client.list()?;
    println!("{:?}", records);

    Ok(())
}
