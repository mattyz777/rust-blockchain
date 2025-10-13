# Result

- Result is a system defined enum providing Ok and Err variant.
- `Result<i32, String>` firs parameter is Ok variant, second is Err variant.

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

# Result & ?

- The ? operator only works on a Result (or Option).
- It unwraps Ok(value) and returns the value, or returns early if it’s an Err.
- It does not care about .await at all.

```rs
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn calculate(a: f64, b: f64, c: f64) -> Result<f64, String> {
    // if OK => result1 is f64 value
    // if Err => directly returns Err
    let result1 = divide(a, b)?;
    let result2 = divide(result1, c)?;
    Ok(result2)
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
