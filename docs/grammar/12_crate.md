# types of crate

- binary crate → `src/main.rs` → built executable(`.exe` etc)
- library crate → `src/lib.rs` → built library(`.rlib`)

# default crates

- Binary crate(`src/main.rs`) → produces an executable (.exe, etc.)
- Library crate(`src/lib.rs`) → produces a reusable library (.rlib)
- Both exist `src/main.rs` and `src/lib.rs`,
  - `main.rs` defines entry point `main()`.
  - `lib.rs` defines library crate, containing reusable modules and functions.
  - The binary (`main.rs`) can import/use everything from `lib.rs`.

# example

```rs
// current file is main.rs
// myapp is the name defined in Cargo.toml
// greet is the mod or file name defined in src/lib.rs
use myapp::greet;

fn main() {
  greet::hello();
}
```

```toml
# Cargo.toml
[package]
name = "myapp"
```

```rs
// src/lib.rs
mod greet;
```

```rs
// src/greet.rs
pub fn hello() {}
```

# crate rules

- one package could have 0-1 library crate
- one package could have 0-many binary crates
