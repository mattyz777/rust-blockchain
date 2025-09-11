# structure

```
rust_blockchain/
├── Cargo.toml      <-- Workspace's manifest file
├── app_project/    <-- crate
│   ├── Cargo.toml  <-- 'app_project' crate's manifest
│   └── src/
│       └── main.rs
└── matt_lib/       <-- crate
    ├── Cargo.toml  <-- 'matt_lib' crate's manifest
    └── src/
        └── lib.rs
```

# rust_blockchain/Cargo.toml

```toml
[workspace]
members = [
    "app_project",
    "matt_lib"
]
```
