# summary

- `mod` 用于加载指定的模块文件（或代码块）；然后通过 `use` 引入。
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
```

```rs
// utils/mod.rs
pub mod helper;
```

```rs
// main.rs
mod utils;                         // no matter #1 or #2, both need mod utils

fn main() {
    use utils::helper::greet;      // #1 ✅ 必须用 use 导入模块或函数
    greet("Matt");                 // #1

    utils::helper::greet("Matt");  // #2
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
