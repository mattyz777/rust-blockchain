# clone a template

- `cargo install cargo-generate` installs the tool cargo-generate.exe globally.
- `cargo generate <template>` runs the tool to create a new project from the specified template.

# create a project via cmd

`cargo new myapp`

# create a project manually

## structure

```
myapp/
 ├─ Cargo.toml
 └─ src/
     └─ main.rs
```

## Cargo.toml

```toml
[package]
name = "myapp"
version = "0.1.0"
edition = "2021"

[dependencies]
```

## src/main.rs

```rs
fn main() {
    println!("Hello, world!");
}
```

# run project

```
cargo build
cargo run

or

.\target\debug\myapp.exe
```

# dependencies installation position

```
%USERPROFILE%\.cargo\registry\
```
