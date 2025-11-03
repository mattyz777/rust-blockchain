# Result

- The `?` operator can only be used inside functions that return a `Result` or `anyhow::Result`.
- If you call a function that returns a `Result` or `anyhow::Result` without using `?`, the error will be ignored (thrown away) unless you explicitly handle it.
- Use `.unwrap()` or `?` to extract the value from a Result.
- When a function returns `Result<T, E>`, returning a plain value of type `T` will be automatically wrapped as `Ok(T)`.
- Use `Err("...")` or `Err(anyhow!("..."))` when using anyhow to return an error.
- Use `match` to handle `Result` or custom errors explicitly.

# Result returns early

"Returns early" means that a function stops executing before reaching its final return statement or the last expression, and immediately hands control back to the caller, usually because of some condition or error.

```rs
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("division by zero".to_string());
    }

    Ok(a / b)
}

fn main() {
    let result = divide(10, 0);  // returns early with Err
    println!("{:?}", result);    // Output: Err("division by zero")
}
```

# ？ vs unwrap

- `?`: When encountering an `Err`, it returns early from the current function and propagates the error upward. It does not panic.
- `unwrap`: When encountering an `Err`, it panics immediately. Suitable for prototyping, testing, or when you are certain the `Result` is `Ok`.

```rs
use anyhow::Result;

pub fn caller() -> Result<String> {
    // Calling without `?` ignores the error:
    let _ = worker(); // error is thrown away

    // Using `?` will propagate the error early:
    let val = worker()?; // if Err, caller() returns early with the error

    Ok(format!("Success: {}", val))
}

pub fn worker() -> Result<String> {
    Err(anyhow::anyhow!("worker error"))
}
```

# Result & Match

```rs
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}
```
