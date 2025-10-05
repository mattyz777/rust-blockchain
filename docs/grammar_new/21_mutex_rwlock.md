# Mutext<T>

- Mutual exclusion: only one thread can access the data at a time.
- Use `.lock()` to acquire the lock.

```rs
use std::sync::Mutex;

let data = Mutex::new(0);                 // Create a Mutex protecting the integer 0

{
    let mut num = data.lock().unwrap();   // Attempt to acquire the lock
    // Lock acquired successfully, safe to modify the data
    *num += 1;                            // Internal data becomes 1
    // `num` (MutexGuard) goes out of scope, lock is automatically released
}

// At this point, another thread can acquire the lock
```

# RwLock<T>

- Read-write lock: multiple readers can access the data simultaneously, but writers are exclusive.
- Use `.read()` for shared access and `.write()` for exclusive access.

```rs
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(RwLock::new(0));           // ownership of the lock object

    let mut handles = vec![];
    for i in 0..5 {
        let data = Arc::clone(&data);              // reference plus 1
        handles.push(thread::spawn(move || {
            loop {
                let val = *data.read().unwrap();   // acquire read lock
                println!("Reader {}: {}", i, val);
                thread::sleep(Duration::from_millis(500));
            }
        }));
    }

    let writer = thread::spawn(move || {
        for i in 1..5 {
            thread::sleep(Duration::from_secs(2));
            {
                let mut w = data.write().unwrap();  // acquire write lock
                *w += 10;
                println!("==== Writer updated to {} ====", *w);
            } // auto release lock here
        }
    });

    thread::sleep(Duration::from_secs(10));
    std::process::exit(0);
}
```
