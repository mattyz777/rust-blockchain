# generic functions

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

# generic struct

```rust
impl<T> Option<T> {
    fn is_some_and(&self) -> bool {
        matches!(self, Some(_))
    }
}
```

# generic enum

```rust
#[derive(Debug)]
enum ResponseType<T> {
    Success {
        code: u16,
        data: T,
        message: String,
    },
    Failure {
        code: u16,
        message: String,
    },
}

// Response containing a String payload
let r1: ResponseType<String> = ResponseType::Success {
    code: 200,
    data: "User info loaded".to_string(),
    message: "OK".to_string(),
};

// Response containing a Vec<i32> payload
let r2: ResponseType<Vec<i32>> = ResponseType::Success {
    code: 200,
    data: vec![1, 2, 3],
    message: "Numbers retrieved".into(),
};

// Failure without data
let r3: ResponseType<()> = ResponseType::Failure {
    code: 404,
    message: "Not found".into(),
};
```
