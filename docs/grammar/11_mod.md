# summary

- `mod` 用于加载指定的模块文件（或代码块）；然后通过 `use` 引入。
- `use ::{m, n};` 引入模块中的指定内容，如多个函数、对象
- `mod x;` x is file name and Rust searches in the same directory as the current file for one of these:
  - `x.rs`
  - `x/mod.rs`
- `x::xxx` xxx is something inside file x.rs or x/mod.rs

# mod and use

```
src/
├── main.rs
└── utils/
    ├── mod.rs
    └── helper.rs
```

```rs
// utils/helper.rs
pub fn greet(name: &str) {
    println!("Hello, {name}!");
}

pub fn bye(name: &str) {
    println!("Goodbye, {name}!");
}
```

```rs
// utils/mod.rs
pub mod helper;
```

```rs
// main.rs
mod utils;                             // no matter #1 or #2, both need mod utils

fn main() {
    use utils::helper::{greet, bye};   // #1 ✅ 必须用 use 导入模块或函数
    greet("Matt");                     // #1
    bye("Matt");                       // #1

    utils::helper::greet("Matt");      // #2
    utils::helper::bye("Matt");        // #2
}
```

# same directory -> layout

```
src/
 ├── a.rs
 ├── x.rs
```

# same directory -> x.rs

```rs
pub fn test() { }
```

# same directory -> a.rs

```rs
mod x;

fn main() {
  x::test();
}
```

# subdirectory -> layout

```
src/
 ├── a.rs
 └── x/
     ├── mod.rs
     └── y.rs
```

# subdirectory -> y.rs

```rs
pub fn y_hello() { }
```

# mod subdirectory -> mod.rs

```rs
pub mod y;

pub fn m_test() { }
```

# subdirectory -> a.rs

```rs
mod x;

fn main() {
    x::test();
    x::y::hello();
}
```

# Compiler Looks in Wrong Path

```
src/
 ├── main.rs
 ├── app_state.rs
 └── dtos/
     ├── auth_dtos.rs
     └── user_dtos.rs
```

```rs
// file app_state.rs

// mod dtos;
```

`mod dtos;` tells the compiler to look for a dtos module inside the app_state module (i.e., at src/app_state/dtos.rs), which doesn't exist.

# Relationship Between main.rs and lib.rs

When both `main.rs` and `lib.rs` exist, one package with two crates will be built.

- `lib.rs` → builds a library crate (my_app).
- `main.rs` → builds a binary crate (the executable).
- The binary crate links to the library crate, producing a single executable in the final build.

# Crate Path Usage - "crate::" vs "<app_name>::"

- Inside the same crate (e.g., within `lib.rs` or its submodules) → use `crate::...`
- Outside the crate (e.g., `main.rs` or tests accessing `lib.rs`) → use `<app_name>::...`
- When centralizing modules in lib.rs, you must reference them as `<app_name>::module` in `main.rs`.
- If there’s no `lib.rs` (only a binary app), `mod xxx` in `main.rs` is enough to include local modules.
