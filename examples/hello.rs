extern crate jsonbox;

use jsonbox::{Client, Error};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize)]
struct Greeting {
    name: String,
    message: String,
}

fn main() -> Result<(), Error> {
    let client = Client::new("kuy_ed82aef3f93176996146");

    let all = client.read().all::<Greeting>()?;
    if let Some((record, meta)) = all.first() {
        println!(
            "Greeting from {} at {}: {}",
            record.name.trim(),
            meta.created_on.trim(),
            record.message.trim(),
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
