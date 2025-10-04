# tokio scheduler modes concurrency vs parallelism

Tokio's scheduler choose whether tasks run concurrently on a single thread or parallelism across multiple threads, depending on whether the task is yielding or actively computing.

- Concurrency (Same Thread): When a task encounters a waiting state (e.g., I/O or a timer) and hits `.await`, it voluntarily yields control. The underlying OS thread remains unblocked and is immediately reassigned by the Executor to process another ready task. This allows a single OS thread to concurrently manage thousands of non-blocking I/O operations.
- Parallelism (Different Threads): The Executor distributes ready tasks across its entire thread pool, enabling multiple tasks to run simultaneously on different CPU cores.

# tokio Scheduler Mechanism

Tokio scheduler takes the submitted task, processes it using its thread pool, and delivers the final result back to the awaiting caller. Tokio scheduler oincludes two components:

- The Executor (The Scheduler): It functions similarly to an event loop, taking submitted tasks (Futures/coroutines) and placing them on a ready queue. It is responsible for actively polling these tasks to drive them forward.
- The Reactor (I/O Polling): This component integrates with the operating system to monitor external events (e.g., a network socket receiving data, a timer expiring). When an event occurs, the Reactor signals the Executor, which then uses the task's Waker to mark the suspended task as "ready" and put it back onto the queue for a worker thread to resume.

# tokio main

main function works as a task and is delivered to the Tokio Executor, meaning that main function is not run on the current thread but instead runs on a worker thread.

```rs
#[tokio::main]
async fn main() {
}
```

# tokio process

- Future Creation
  - The `async { ... }` block is compiled into a lightweight Future.
- Delivery to Executor
  - The `tokio::spawn` function takes the Future and submits it to the central Tokio Executor(scheduler).
- Ready Queue
  - The Executor places the new task onto its internal ready queue.
- Initial Polling
  - A worker thread from the thread pool pulls the task off the queue and begins polling it (running its code).
- Synchronous Execution
  - The synchronous portion of the task is run: `println!("[Task A] Starting long sleep");`. This runs entirely on the worker thread that polled the task.
- Voluntary Yield
  - The task hits `tokio::time::sleep(Duration::from_secs(1)).await;`. The task saves its state, notifies the Reactor (Tokio's I/O watcher), and yields control back to the Executor. The worker thread is immediately free to process other tasks.
- Resumption Polling
  - After the 1-second delay, the Reactor signals the Executor that the timer is complete. The Executor marks Task A as "ready" and places it back onto the ready queue.
- Final Execution
  - A worker thread (which is likely not the same thread that started Task A) pulls the ready task, resumes polling, and runs the final synchronous code `println!("[Task A] Task A is done.");`.
- Task Completion & Waking
  - Task A finishes, its return value ("Task A completed successfully") is stored in the JoinHandle, and the Executor uses the Waker mechanism to mark the suspended main task (waiting at `task_a.await`) as ready for resumption.

```rs
let task_a = tokio::spawn(async {
    println!("[Task A] Starting long sleep");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("[Task A] Task A is done.");

    "Task A completed successfully".to_string()
});
let result_a = task_a.await.expect("Task A failed to run."); // blocks here until Task A is done.
```

# tokio sequential

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rs
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("--- Program Start ---");

    let task_a = tokio::spawn(async {
        println!("[Task A] Starting long sleep");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("[Task A] Task A is done.");

        "Task A completed successfully".to_string()
    });

    async fn task_b() {
        println!("[Task B] Starting short sleep");
        tokio::time::sleep(Duration::from_millis(10)).await;
        println!("[Task B] Task B is done.");
    }


    task_b().await; // blocks here until Task B is done.
    let result_a = task_a.await.expect("Task A failed to run."); // blocks here until Task A is done.

    println!("Task A result: {}", result_a);
    println!("--- Program End ---");
}

/**
--- Program Start ---
[Task A] Starting long sleep
[Task B] Starting short sleep
[Task B] Task B is done.
[Task A] Task A is done.
Task A result: Task A completed successfully
--- Program End ---
*/
```

# tokio parallel

Because both Task A and Task B run concurrently in the Tokio thread pool, we cannot guarantee which worker thread picks up which task first.

```rs
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("--- Program Start ---");

    let task_a = tokio::spawn(async {
        println!("[Task A] Starting long sleep");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("[Task A] Task A is done.");

        "Task A completed successfully".to_string()
    });

    async fn task_b() {
        println!("[Task B] Starting short sleep");
        tokio::time::sleep(Duration::from_secs(10)).await;
        println!("[Task B] Task B is done.");
    }


    let (result_a, result_b) = tokio::join!(task_a, task_b);

    println!("Task A result: {}", result_a);
    println!("--- Program End ---");
}

/**
--- Program Start ---
[Task A] Starting long sleep
[Task B] Starting short sleep
[Task B] Task B is done.    <-------------------------------
[Task A] Task A is done.    <-------------------------------
Task A result: Task A completed successfully
--- Program End ---

or

--- Program Start ---
[Task A] Starting long sleep
[Task B] Starting short sleep
[Task A] Task A is done.    <-------------------------------
[Task B] Task B is done.    <-------------------------------
Task A result: Task A completed successfully
--- Program End ---
*/
```
