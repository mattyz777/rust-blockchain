# install

```bash
cargo install anyhow
```

# usage

- The ? operator can only be used inside a function that returns a Result (or Option).
- The ? operator automatically converts the error into anyhow::Error (via From) and returns it.
- It needs explicitly return Err(...) if a custom error message is needed.

```rs
use anyhow::{Result, Context};

fn test() -> Result<i32> {
    let s = "42";
    let num: i32 = s.trim().parse().context("Failed to parse number")?;

    Ok(num)
}
```

```rs
if user.is_none() {
    return Err(anyhow::anyhow!("User not found"));
}
```
