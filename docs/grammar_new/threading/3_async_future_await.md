# async Future await

- Rust provides a design to execute asynchronous code through the `async`/`.await`/`Futures`.
- Rust doesn't provide the engine/executor to run asynchronous code directly.
- Must leverage 3rd party library to execute asynchronous code, e.g. tokio

# async

```rs
async fn get_number() -> u32 {
    42
}

async fn run() {
    let future = get_number();        // Future<Output = u32>
    // Because get_number() never fails and its return type is u32, there’s no need for Result & ?.
    let result: u32 = future.await;   // `.await` returns 42
}
```

# Future

- Represents an asynchronous tk computation that may not be ready yet but promises to produce a result
- It is non-blocking. When the operation must wait (e.g., for network I/O), it yields control of the thread to the executor rather than blocking it.

# await

- can only be used in async fn or async { ... } blocks
- Used to consume a Future and wait for its result by repeatedly polling it via the executor.

# example

```rs
async fn a() -> String {
    "Hello".to_string()
}

async fn run_and_await() -> u32 {
    let a_future = a();               // This is a Future<Output = String>
    let b_future = async { 42 };      // This is a Future<Output = u32>

    let result: u32 = a_future.await; // <-- Execution pauses here
    let result: u32 = b_future.await; // <-- Execution pauses here

    result
}
```
