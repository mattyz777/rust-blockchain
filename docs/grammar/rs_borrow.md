# 同一内存块，只能有一个可变引用，支持多个可读引用

- At any given time, you can have either one mutable reference or any number of immutable references to a value.

```rs
let mut ss = String::from("aaa");
let ss1 = &mut ss;   // <-- mutable borrow
let ss2 = &mut ss;   // <-- mutable borrow again
println!("{} {}", ss1, ss2); // compilation error on ss1
```

```rs
let mut ss = String::from("aaa");
let ss2 = &mut ss;    // <-- mutable borrow
ss.push_str("bbb");   // <-- mutable borrow again since ss.push_str() changes string which requires mutable reference
println!("{} {}", "ss1", ss2); // <-- still using ss2 here
```
