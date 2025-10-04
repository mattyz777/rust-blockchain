# background

- In rust, Data can have only one owner at a time.
- If you want to share data, you typically borrow it using a reference (&).
- Simple borrowing is unsafe across threads because the first thread might drop the data while the second thread is still using a reference to it. This leads to a use-after-free error.

# Arc

ensures shared "ownership" and lifetime, allowing the data to be safely accessed and modified across multiple threading contexts.

# Arc readonly int

- counter is i32 which impl Copy trait, when perform `move`, rust will copy the value.
- `let mut counter = 0;` doesn't work even though i32 is Copy.

```rs
let counter = 0;

let handle1 = thread::spawn(move || {
    println!("Task1 : {}", counter);
});

let handle2 = thread::spawn(move || {
    println!("Task2: {}", counter);
});

handle1.join().unwrap();
handle2.join().unwrap();
```

# Arc readonly String wrong

- name is String which doesn't impl Copy trait. when perform `move`, rust will move the value.

```rust
let name = "hello".to_string();

let h1 = thread::spawn(move || {
    println!("{}", name);
});

let h2 = thread::spawn(move || {
    println!("{}", name);
});
```

# Arc readonly String correct

- `Arc::new(shared_data)`
- `Arc::clone(&shared_data_arc)`

```rs
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let shared_data = String::from("Hello, World!");
    let shared_data_arc = Arc::new(shared_data);

    let mut handles = vec![];

    for i in 0..5 {
        let shared_data_arc = Arc::clone(&shared_data_arc); // shadowing
        let handle = tokio::spawn(async move {
            sleep(Duration::from_secs(2)).await;
            println!("Task {}: {}", i, shared_data_arc);
            format!("Task {}: {}", i, shared_data_arc)
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.unwrap();
        println!("Result: {}", result);
    }

    println!("--- program end ---");
}
```

# Mutext

enables atomicity and visibility, ensuring that shared data is modified correctly by only one thread at a time.

# Arc Mutext mutable

- `Arc::new(Mutex::new(0));`
- `Arc::clone(&counter);`
- `let mut num = counter.lock().await;`
- `let result = *counter.lock().await;`

```rs
use tokio::sync::Mutex;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);

        let handle = tokio::spawn(async move {
            let mut num = counter.lock().await;
            *num += 1;
            println!("Task {}: {}", i, *num);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let result = *counter.lock().await;
    println!("Result: {}", result); // Result: 5
}
```
