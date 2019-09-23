extern crate jsonbox;

use jsonbox::{Client, Error};
use serde::{Deserialize, Serialize};

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
    let client = Client::new("box_ed82aef3f93176996145");

    let mut data = Data::new("kuy", "Hello, Jsonbox!");
    let (record, meta) = client.create(&data)?;
    println!("CREATE: data={:?}, meta={:?}", record, meta);

    let (record, meta) = client.read().id::<Data>(&meta.id)?;
    println!("READ: data={:?}, meta={:?}", record, meta);

    let all = client.read().all::<Data>()?;
    println!("READ: len={}, all={:?}", all.len(), all);

    let asc = client.read().order_by("createdOn").run::<Data>()?;
    println!("READ: len={}, asc={:?}", asc.len(), asc);

    let few = client.read().limit(3).run::<Data>()?;
    println!("READ: len={}, few={:?}", few.len(), few);

    data.message = format!("Hello, GitHub! [{}]", meta.id);
    let _ = client.update(&meta.id, &data)?;
    println!("UPDATE: OK");

    let _ = client.delete(&meta.id)?;
    println!("DELETE: OK");

    Ok(())
}
