# install

```bash
cargo add bcrypt
```

# usage

```rs
fn main() {
    let hashed_password = bcrypt::hash("123", bcrypt::DEFAULT_COST).unwrap();
    println!("{}", hashed_password);
}
```
