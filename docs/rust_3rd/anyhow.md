# install

```bash
cargo install anyhow
```

# usage

The ? operator can only be used inside a function that returns a Result (or Option).

```rs
use anyhow::{Result, Context};

fn test() -> Result<i32> {
    let s = "42";
    let num: i32 = s.trim().parse().context("Failed to parse number")?;

    Ok(num)
}
```
