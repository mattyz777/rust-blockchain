# `{}` and `{:?}`

- `{}` → calls fmt from `Display`, custom struct doesn't implement `Display` by default, so should use `[derive[Debug]]`
- `{:?}` `{:#?}` → calls fmt from `Debug`

# Display

- defined in `std::fmt::Display`
- struct cannot use `{}` since it requires `Display`

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
