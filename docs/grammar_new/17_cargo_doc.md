# opt-level

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

# documentation

- `lib.rs`
- `\target\doc\rcli\index.html`

`///` will can be turned into html

````rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_create::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
````

# cargo doc

If both main.rs and lib.rs exist

- cargo build / cargo run builds the binary (main.rs).
- cargo doc documents only the library crate (lib.rs), not the binary.
