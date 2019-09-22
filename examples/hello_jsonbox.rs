extern crate jsonbox;

use jsonbox::{Client, Error};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize)]
pub struct Greeting {
    pub name: String,
    pub message: String,
}

fn main() -> Result<(), Error> {
    let client = Client::new("kuy_ed82aef3f93176996146");

    let records: Vec<Greeting> = client.read_all()?;
    if let Some(record) = records.first() {
        println!(
            "Greeting from {}: {}",
            record.name.trim(),
            record.message.trim()
        );
    } else {
        println!("No message left, you're the first.");
    }

    println!("What is your name?");
    let mut name = String::new();
    let _ = io::stdin().read_line(&mut name);

    println!("Leave message for next guest :)");
    let mut message = String::new();
    let _ = io::stdin().read_line(&mut message);

    let data = Greeting {
        name: name.trim().to_string(),
        message: message.trim().to_string(),
    };
    let _ = client.create(&data)?;
    println!("Thank you!");
    Ok(())
}
