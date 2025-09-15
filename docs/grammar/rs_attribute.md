# attribute

- terms
  - `#[derive(Debug, Clone)]` is an attribute
  - `derive` is a mechanism
  - `Debug, Clone` are traits
- When `#[derive(Debug, Clone)]`, the compiler automatically generates the implementations of those traits for your type — for example, for Debug

```rust
impl std::fmt::Debug for Teacher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Teacher")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}
```

# `getset`

```rust
use getset::{Getters, Setters};

#[derive(Getters, Setters, Debug, Clone)]
pub struct Person {
    #[getset(get = "pub", set = "pub")]
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
