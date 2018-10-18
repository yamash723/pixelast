pixelast
=================

This library is Pixela client for Rust.

* [Pixe.la](https://pixe.la/)
* [Documentation](https://docs.rs/pixelast)

Usage
--------

Add this to your `Cargo.toml`

```
[dependencies]
pixelast = "0.1.0"
```

Example
---------

```rust
use pixelast::{PixelaClient, ConsentAnswer, GraphType, GraphColor};

fn main() {
    let res = PixelaClient::create_new_user(
        "username",
        "usertoken",
        ConsentAnswer::Yes,
        ConsentAnswer::Yes
    );

    match res {
        Ok(()) => println!("create new user."),
        Err(v) => panic!("create new user failed. {}", v),
    }

    let client = PixelaClient::new("username", "usertoken");
    client.create_graph("graphid", "graphname", "cal", GraphType::Int, GraphColor::Shibafu).unwrap();

    client.record_pixel("graphid", "20181017", "10").unwrap();
    let svg = client.get_graph_svg("graphid", Some("20181020")).unwrap();
    
    println!("{}", svg);
}
```

**User**

```rust
// Create new user.
let res = PixelaClient::create_new_user(
    "username",
    "usertoken",
    ConsentAnswer::Yes,
    ConsentAnswer::Yes
);
```

```rust
// Update user token.
let client = PixelaClient::new("username", "usertoken");
client.update_user_token("newusertoken").unwrap();
```

```rust
// Delete user.
let client = PixelaClient::new("username", "usertoken");
client.update_user_token("newusertoken").unwrap();
```

**Graph**

```rust
// Create graph.
let client = PixelaClient::new("username", "usertoken");
client.create_graph("graphid", "graphname", "cal", GraphType::Int, GraphColor::Shibafu).unwrap();
```

```rust
// Update graph definition.
let client = PixelaClient::new("username", "usertoken");
client.update_graph("graphid", "graphname", "kcal", GraphColor::Shibafu).unwrap();
```

```rust
// Delete graph.
let client = PixelaClient::new("username", "usertoken");
client.delete_graph("graphid").unwrap();
```

```rust
// Get all graphs.
let client = PixelaClient::new("username", "usertoken");
let graphs = client.get_graphs().unwrap();
println!("{:?}", graphs);
```

```rust
// Get graph SVG.
let client = PixelaClient::new("username", "usertoken");
let svg = client.get_graph_svg("graphid", Some("20181020")).unwrap();
// let svg = client.get_graph_svg("graphid", None).unwrap();
println!("{}", svg);
```

```rust
// Get graph SVG.
let client = PixelaClient::new("username", "usertoken");
let svg = client.get_graph_svg("graphid", Some("20181020")).unwrap();
// let svg = client.get_graph_svg("graphid", None).unwrap();
println!("{}", svg);
```

**Pixel**

```rust
// Record pixel.
let client = PixelaClient::new("username", "usertoken");
client.record_pixel("graphid", "20181016", "10").unwrap();
```

```rust
// Update pixel.
let client = PixelaClient::new("username", "usertoken");
client.update_pixel("graphid", "20181018", "20").unwrap();
```

```rust
// Delete pixel.
let client = PixelaClient::new("username", "usertoken");
client.delete_pixel("graphid", "20181016").unwrap();
```

```rust
// Get pixel quantity.
let client = PixelaClient::new("username", "usertoken");
let pixel = client.get_pixel("graphid", "20181018").unwrap();
println!("{:?}", pixel);
```

```rust
// Increment pixel.
let client = PixelaClient::new("username", "usertoken");
client.increment("graphid").unwrap();
```

```rust
// Decrement pixel.
let client = PixelaClient::new("username", "usertoken");
client.decrement("graphid").unwrap();
```
