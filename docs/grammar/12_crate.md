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

# crate-level attribute

- `#![attribute_name(arguments)]`
- `#![no_std]` the crate should not link the standard library (std), making it suitable for embedded systems or operating system kernels.

# crate-level attribute - cfg

The crate-level `cfg` attribute (`#![cfg(condition)`) is used for conditional compilation

```rs
// This module is only included if the target OS is Linux.
#[cfg(target_os = "linux")]
pub mod linux_networking;

// This module is only included if the target OS is Windows.
#[cfg(target_os = "windows")]
pub mod windows_networking;
```

```bash
cargo build --target x86_64-pc-windows-msvc
```

# custom feature

```toml
# Cargo.toml

[features]
abc = []
```

```rs
#[cfg(feature = "abc")]          // Compile-Time Filter, if the abc feature is not enabled when compiling, the Rust compiler (rustc) will completely remove the run_special_feature function
pub fn run_special_feature() {
    println!("The custom 'abc' feature is enabled!");
}

pub fn main() {
    if cfg!(feature = "abc") {   // Compile-Time Value Injection,
                                 // If it is enabled: `if cfg!(feature = "abc")` becomes `if true`.
                                 // Esle, `if false`.
        run_special_feature();
    } else {
        println!("The custom 'abc' feature is NOT enabled.");
    }
}
```

```bash
carog build                      # false
cargo build --features **abc**   # true
cargo build --all-features       # true
cargo test --features **abc**    # true
```
