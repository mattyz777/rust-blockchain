# basic

```rs
let o1:Option<i32> = Some(5);
let o2:Option<&str> = Some("hello");
let o3:Option<i32> = None;
```

# Option extract with unwrap_or (default value)

```rs
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y.unwrap_or(0);
```
