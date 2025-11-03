# panic

- `unwrap` will trigger panic if there is an error
- explicilty call `panic!` to trigger panic

# handle in single thread

```
----1--> PANIC detected: panicked at src\main.rs:12:5:
----2--> Database connection failed!
----3--> Recovered from panic. Proceeding with cleanup...
----4--> App did not crash. Still running cleanly.
```

```rs
// main.rs
use std::panic;

fn risky_business_logic() {
    panic!("----2--> Database connection failed!");
}

fn main() {
    panic::set_hook(Box::new(|info| {
        eprintln!("----1--> PANIC detected: {info}");
    }));

    let result = panic::catch_unwind(|| {
        risky_business_logic();
    });

    // following code 3/4 are not necessary
    match result {
        Ok(_) => println!("successfully."),
        Err(_) => {
            eprintln!("----3--> Recovered from panic. Proceeding with cleanup...");
        }
    }

    println!("----4--> App did not crash. Still running cleanly.");
}
```

# handle in multi threading

```
----1--->  Global PANIC detected: panicked at src\main.rs:5:5:
----2--->  Worker task crashed unexpectedly!
----3--->  Worker panicked, but main thread recovered.
----4--->  Cause: ----2--->  Worker task crashed unexpectedly!
----5--->  Main thread continues running normally.
```

```rs
fn main() {
    panic::set_hook(Box::new(|info| {
        eprintln!("----1--->  Global PANIC detected: {info}");
    }));

    let handle = thread::spawn(|| {
        business_worker();
    });

    // way 1
    handle.join();

    // way 2
    // match handle.join() {
    //     Ok(_) => println!("✅ Worker finished successfully."),
    //     Err(err) => {
    //         eprintln!("----3--->   Worker panicked, but main thread recovered.");
    //         if let Some(msg) = err.downcast_ref::<&str>() {
    //             eprintln!("----4--->  Cause: {msg}");
    //         }
    //     }
    // }

    // println!("----5--->   Main thread continues running normally.");
}
```

# clippy

```
cargo clippy -- -D warnings
```

```toml
[workspace.metadata.clippy]
deny = ["unwrap_used", "expect_used", "panic"]
```

# fuzzy

```
cargo install cargo-fuzz
cargo fuzz init
cargo fuzz run my_target
```

# thread safe function

```
worker-1: starting normal work...
worker-2: doing risky operation...
❌ Thread 'worker-2' panicked: Any { .. }
worker-1: done without errors!
✅ Thread 'worker-1' finished successfully.
Main thread continues running cleanly.
```

```rs
use std::{thread, time::Duration, panic};

fn safe_thread<F>(name: &str, f: F) -> thread::JoinHandle<()>
where
    F: FnOnce() + Send + 'static,
{
    let name = name.to_string();
    thread::spawn(move || {
        if let Err(err) = panic::catch_unwind(f) {
            eprintln!("❌ Thread '{name}' panicked: {:?}", err);
        } else {
            println!("✅ Thread '{name}' finished successfully.");
        }
    })
}

fn main() {
    let handle1 = safe_thread("worker-1", || {
        println!("worker-1: starting normal work...");
        thread::sleep(Duration::from_secs(1));
        println!("worker-1: done without errors!");
    });

    let handle2 = safe_thread("worker-2", || {
        println!("worker-2: doing risky operation...");
        panic!("database connection failed!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Main thread continues running cleanly.");
}
```
