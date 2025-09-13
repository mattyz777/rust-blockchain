# marco

# #[derive(Debug)]

- `#[derive(Debug)]` is built-in macro used to print out struct data. It tells the Rust compiler to automatically implement the Debug trait for your struct (or enum).
- it can use on struct, enum, tuple struct

```rust
#[derive(Debug)]
struct Teacher {
    name: String,
    age: i32,
}

// This expands (behind the scenes) into an implementation roughly like:

impl std::fmt::Debug for Teacher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Teacher")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}
```

# #[allow(dead_code)]

```rust
#[allow(dead_code)]  // tells compiler: don't warn if this fn is unused
fn unused_function() {}
```
