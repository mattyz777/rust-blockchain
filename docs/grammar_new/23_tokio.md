# tokio

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rs
use tokio::time::{sleep, Duration};

// 返回 impl Future<Output = i32>
async fn greet_and_wait(id: i32, seconds: u64) -> i32 {
    println!("Task {}: 开始等待 {} 秒...", id, seconds);

    // 注意：tokio::time::sleep 返回一个 Future
    sleep(Duration::from_secs(seconds)).await;
    // let response = reqwest::get(&url).await?;
    // sqlx::query!(...).execute(pool); // 返回 future


    // 等待结束后，任务被唤醒，继续执行
    println!("Task {}: 等待结束，继续执行。", id);

    id * 10
}


#[tokio::main]
async fn main() {
    let task1_future = greet_and_wait(1, 3); // 返回 Future
    let task2_future = greet_and_wait(2, 1); // 返回 Future

    println!("main: 开始并发等待...");

    let (result1, result2) = tokio::join!(task1_future, task2_future);

    println!("--- 所有任务完成 ---");
    println!("结果 1 (Future::Output): {}", result1); // 1 * 10 = 10
    println!("结果 2 (Future::Output): {}", result2); // 2 * 10 = 20
}
```
