# init

```
mkdir proj_crm
cd proj_crm
```

# proj_crm/Cargo.toml

```toml
[workspace]
members = [
    "crm_app",
    "crm_lib"
]
```

# create project structure

```
# proj_crm
cargo new crm_app
cargo new crm_lib --lib
```

# structure

```
proj_crm/
├── Cargo.toml       <-- Workspace/package's manifest file
├── crm_lib/         <-- lib crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── contact/
│       │   ├── mod.rs
│       │   ├── contact.rs
│       │   ├── profile.rs
│       │   └── tag.rs
│       └── order/
│           ├── mod.rs
│           ├── order.rs
│           ├── header.rs
│           └── line.rs
└── crm_app/          <-- bin crate
    ├── Cargo.toml
    └── src/
        └── main.rs
```

# crm_app use crm_lib method

- crm_app/Cargo.toml

```toml
[dependencies]
crm_lib = { path = "../crm_lib" }
```

- crm_app/src/main.rs

```rust
fn main() {
    crm_app::contact::profile::some_function();
}
```

# crm_lib/src/lib.rs

```rs
pub mod contact;
```

# crm_lib/src/contact/mod.rs

```rs
pub mod contact;
pub mod profile;
pub mod tag;
```

# run

```
cd proj_crm
cargo run -p crm_app
```
