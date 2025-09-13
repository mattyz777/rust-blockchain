# Dereferencing

Dereferencing is the process of accessing the value that a reference points to.

# Implicitly dereferences

```rust
let x = 5;
let y = &x; // y is a reference to x (&i32)

// implicitly dereferences `y` to call the `abs()` method on the `i32` type.
let abs_value = y.abs();

```
