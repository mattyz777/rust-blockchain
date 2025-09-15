# `{}` and `{:?}`

- `{}` → calls fmt method from `Display` trait, custom struct doesn't implement `Display` by default
- `{:?}` `{:#?}` → relies on `Debug` trait

# Display

- defined in `std::fmt::Display`

```rust
let p = Point { x: 3, y: 4 };
println!("{}", p); // Error
```

## Display for struct

```rust
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    // `{}` uses the `Display` implementation
    println!("{}", p);   // → Point(3, 4)
}
```

# Debug

- defined in `std::fmt::Debug`
- invoke by `{:?}`, `{:#?}`

```rust
[derive(Debug)]
struct Person { name: String, age: u32 }

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("{:?}", p);   // prints: Point { x: 3, y: 4 }
    println!("{:#?}", p);  // pretty print
}
```
