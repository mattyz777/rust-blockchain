# Atomic

`Arc::new(AtomicUsize::new(0))` doesn’t need a Mutex, because `AtomicUsize` is already thread-safe by itself.

- `Arc::new(...)` → allows to be shared across threads.
- `Arc::clone(&counter)` → each thread gets its own handle (same data, not copied).
- `fetch_add(1, Ordering::Relaxed)` → atomically increments the value; no lock needed.
- `load(Ordering::Relaxed)` → safely reads the current value.

```rs
use std::sync::{Arc, AtomicUsize};
use std::thread;
use std::sync::atomic::Ordering;
use std::time::Duration;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..3 {
        let counter = counter.clone();

        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
                thread::sleep(Duration::from_micros(10));
            }
            println!("thread {} finishes 1000 times increment", i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = counter.load(Ordering::Relaxed);
    println!("result: {}", final_count); // 3000
}
```
