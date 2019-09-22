extern crate jsonbox;

use jsonbox::client::{Client, CreateOp, Error, ListOp};
use std::io;

const BOX_ID: &str = "box_ed82aef3f93176996147";

fn main() -> Result<(), Error> {
    let client = Client::new(BOX_ID);

    let records: ListOp = client.list()?;
    if let Some(record) = records.first() {
        println!("Greeting from previous guest: {}", record.id);
    } else {
        println!("No greeting, you're the first.");
    }

    println!("Please input greeting for next guest.");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);

    let created: CreateOp = client.create()?;
    println!("{}, {}", created.id, created.created_on);
    Ok(())
}
