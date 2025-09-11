# dependencies

| name       | 说明                                  |
| ---------- | ------------------------------------- |
| serde      | serialization and deserialization     |
| serde_json | handle json format                    |
| sha2       | SHA-2 cryptographic hashing algorithm |

# example

```toml
[package]
name = "simple_blockchain"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sha2 = "0.10"
```
