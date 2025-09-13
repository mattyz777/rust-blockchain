# attribute

- `#[derive(Debug, Clone)]` is an attribute
- `Debug, Clone` are traits being derived

# `#[derive(Getters, Setters)]`

- 3rd `getset`

```rust
use getset::{Getters, Setters};

#[derive(Getters, Setters, Debug, Clone)]
pub struct Person {
    #[get = "pub"]       // &String, borrowed
    #[set = "pub"]
    name: String,

    #[get_copy = "pub"]  // u8 implements Copy
    #[set = "pub"]
    age: u8,
}

let mut p = Person {
    name: "Alice".into(),
    age: 30,
};

println!("{:?}", p);     // Debug
let n = p.name();        // &String
let a = p.age();         // u8 (copied, no borrow)
p.set_name("Bob".into());
p.set_age(31);

println!("{} is {} years old", n, a);
```
