# async Future await

- Rust provides a design to execute asynchronous code through the `async`/`.await`/`Futures`.
- Rust doesn't provide the engine/executor to run asynchronous code directly.
- Must leverage 3rd party library to execute asynchronous code, e.g. tokio

# async

- async is used in async fn or async { ... } blocks
- Calling an async fn immediately returns this Future without running the code inside.

```rs
async fn get_data() -> String {
    "Hello".to_string()
}

let my_future = get_data();  // my_future is a lazy Future<Output = String>.
```

# Future

- Represents an asynchronous tk computation that may not be ready yet but promises to produce a result
- It is non-blocking. When the operation must wait (e.g., for network I/O), it yields control of the thread to the executor rather than blocking it.

# await

- can only be used in async fn or async { ... } blocks
- Used to consume a Future and wait for its result by repeatedly polling it via the executor.

# example

```rs
async fn run_and_await() -> u32 {
    let some_num_future = async { 42 }; // This is a Future<Output = u32>

    // .await consumes the Future and returns the u32 value (42)
    let result: u32 = some_num_future.await; // <-- Execution can pause here

    result
}
```
