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
use std::sync::RwLock;

let data = RwLock::new(vec![1, 2, 3]);

// --- Multiple threads can read simultaneously ---
{
    let read_guard1 = data.read().unwrap();
    let read_guard2 = data.read().unwrap();
    println!("Data contents: {:?}", *read_guard1);
    // read_guard1 and read_guard2 go out of scope, read locks are released
}

// --- Writing requires exclusive access ---
{
    let mut write_guard = data.write().unwrap();   // Acquire write lock
    write_guard.push(4);                           // Modify the data
    // write_guard goes out of scope, write lock is released
}
```
