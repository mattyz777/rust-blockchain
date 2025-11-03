# cfg

conditional compilation `#[cfg(boolean expression)]`

```rs
#![no_std]          // 先整个 crate 进 no_std 模式

#[cfg(feature = "std")]
extern crate std;   // 只有开了 std feature 才链接标准库

#[cfg(feature = "std")]
pub fn hello() {
    std::println!("Hello from std!");
}

#[cfg(not(feature = "std"))]
pub fn hello() {
    panic!("Hello from no_std!");
}
```

```bash
cargo build --no-default-features      # 走 no_std 分支
cargo build --features std             # 走 std 分支
```
