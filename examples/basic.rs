extern crate jsonbox;

use jsonbox::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    age: i32,
    login: bool,
}

impl Data {
    fn new(name: &str, age: i32, login: bool) -> Data {
        Data {
            name: name.to_string(),
            age,
            login,
        }
    }
}

fn main() -> Result<(), Error> {
    let client = Client::new("box_8ed82aef3f93176996145");

    let mut data = Data::new("kuy", 42, false);
    let res = client.create(&data)?;
    println!("CREATE: data={:?}, meta={:?}", res.data, res.meta);

    let res = client.read().id::<Data>(&res.meta.id)?;
    println!("READ single: data={:?}, meta={:?}", res.data, res.meta);

    let list = vec![Data::new("jsonbox", 21, false), Data::new("io", 16, true)];
    let bulk = client.create_bulk(&list)?;
    println!("CREATE bulk: len={}, bulk={:?}", bulk.len(), bulk);

    let all = client.read().all::<Data>()?;
    println!("READ: len={}, all={:?}", all.len(), all);

    let asc = client.read().order_by("age").run::<Data>()?;
    println!("READ: len={}, asc={:?}", asc.len(), asc);

    let few = client.read().limit(1).run::<Data>()?;
    println!("READ: len={}, few={:?}", few.len(), few);

    data.name = format!("kuy {}", res.meta.id);
    client.update(&res.meta.id, &data)?;
    println!("UPDATE: OK");

    let filtered = client
        .read()
        .filter_by("name:{}", &data.name)
        .run::<Data>()?;
    println!("READ: len={}, filtered={:?}", filtered.len(), filtered);

    let filtered = client.read().filter_by("name:{}*", "kuy").run::<Data>()?;
    println!("READ: len={}, filtered={:?}", filtered.len(), filtered);

    let filtered = client
        .read()
        .filter_by("name:*{}", &res.meta.id)
        .run::<Data>()?;
    println!("READ: len={}, filtered={:?}", filtered.len(), filtered);

    data.age = 8;
    client.update(&res.meta.id, &data)?;
    println!("UPDATE: OK");

    let filtered = client
        .read()
        .filter_by("age:<{}", 10)
        .and("login:{}", false)
        .run::<Data>()?;
    println!("READ: len={}, filtered={:?}", filtered.len(), filtered);

    client.delete(&res.meta.id)?;
    println!("DELETE: OK");

    Ok(())
}
