# Test attribute

- `#[cfg(test)]` Marks a module that should only be compiled when running tests.
- `#[test]` Marks a function as a test.

# Test Macros / Methods

- `assert!(condition)`
  - `assert!(x > 0); // ✅ Passes;`
  - `assert!(x < 0, "x should be negative, but got {}", x); // ❌ Panics with message`
- `assert_eq!(a, b)`
  - `assert_eq!(sum, 5);  // ✅ Passes`
  - `assert_eq!(sum, 6);  // ❌ Panics with message: left = 5, right = 6`
  - `assert_eq!(sum, 6, "sum was expected to be 6, got {}", sum); // ❌ Panics with message`
- `assert_ne`
  - `assert_ne!(sum, 5);  // ✅ Passes`
  - `assert_ne!(sum, 6);  // ❌ Panics with message: left = 5, right = 6`
  - `assert_ne!(sum, 6, "sum was expected to be 6, got {}", sum); // ❌ Panics with message`

# Unit Tests (in logic code)

```rs
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]       // <------------------
mod tests {
    use super::*;  // Brings parent module items into scope

    #[test]        // <------------------
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]        // <------------------
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }
}
```

# Integration Tests (in tests/ folder)

```
my_crate/
├─ src/
│  ├─ lib.rs
│  ├─ module1.rs
│  ├─ module2.rs
│  └─ ...
├─ tests/          <-- integration tests
│  ├─ module1_tests.rs
│  └─ module2_tests.rs
└─ Cargo.toml
```

```rs
// tests/module1_tests.rs
use my_crate::add;

#[test]
fn test_add_integration() {
    assert_eq!(add(4, 5), 9);
}
```

# run tests

```bash
cargo test                  # Run all tests
cargo test add              # Run tests where method name containing "add"
cargo test -- --nocapture   # Show println output
```
