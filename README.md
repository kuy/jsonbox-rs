# jsonbox.rs

[![crates.io](https://img.shields.io/crates/v/jsonbox.svg)](https://crates.io/crates/jsonbox)
[![docs.rs](https://docs.rs/jsonbox/badge.svg)](https://docs.rs/jsonbox)
[![build](https://github.com/kuy/jsonbox-rs/workflows/build/badge.svg)](https://github.com/kuy/jsonbox-rs/actions)

**⚠️ NOTICE: Alpha quality. Unstable API ⚠️**

⚙️ Rust wrapper for 📦 [jsonbox.io](https://jsonbox.io/).

## Usage

```rust
// Declaration
use jsonbox::{Client, Error};
use serde::{Deserialize, Serialize};

// Define struct
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub message: String,
}

fn main() -> Result<(), Error> {
    // Create client with <BOX_ID>
    let client = Client::new("enjoy_your_first_jsonbox_rs");

    // Put data
    let data = Data {
        name: "kuy".into(),
        message: "Hello, Jsonbox!".into(),
    };
    let (record, meta) = client.create(&data)?;
    println!("CREATE: data={:?}, meta={:?}", record, meta);

    Ok(())
}
```

### CREATE

```rust
let data = Data {
    name: "kuy".into(),
    message: "Hello, Jsonbox!".into(),
};
let (record, meta) = client.create(&data)?;
println!("CREATE: data={:?}, meta={:?}", record, meta);
```

### READ (single)

```rust
let (record, meta) = client.read::<Data>("5d876d852a780700177c0557")?;
println!("READ: data={:?}, meta={:?}", record, meta);
```

### READ (all)

```rust
let all = client.read_all::<Data>()?;
println!("READ: len={}, all={:?}", all.len(), all);
```

### READ (filter) / READ (sort) / READ (limit)

_WIP_

### UPDATE

```rust
let data = Data::new("kuy", "Hello, Jsonbox!");
let _ = client.update("5d876d852a780700177c0557", &data)?;
println!("UPDATE: OK");
```

### DELETE

```rust
let _ = client.delete("5d876d852a780700177c0557")?;
println!("DELETE: OK");
```

## Examples

- [jsonbox-todo-example](https://github.com/kuy/jsonbox-todo-example)
- [hello_jsonbox](https://github.com/kuy/jsonbox-rs/blob/master/examples/hello_jsonbox.rs)
  - `cargo run --example hello_jsonbox`
- [basic](https://github.com/kuy/jsonbox-rs/blob/master/examples/basic.rs)
  - `cargo run --example basic`
- [errors](https://github.com/kuy/jsonbox-rs/blob/master/examples/errors.rs)
  - `cargo run --example errors`

## License

MIT

## Author

Yuki Kodama / [@kuy](https://twitter.com/kuy)
