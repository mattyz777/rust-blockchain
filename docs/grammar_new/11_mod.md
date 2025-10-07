# summary

`mod x;` x is file name and Rust searches in the same directory as the current file for one of these:

- `x.rs`
- `x/mod.rs`

`x::xxx` xxx is something inside file x.rs or x/mod.rs

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
