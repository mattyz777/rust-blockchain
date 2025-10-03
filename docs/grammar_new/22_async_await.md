# async/await 与 Future 执行流程

- 业务逻辑的定义者：`async fn`, 编译器实际返回的是一个 `Future`
  - `async fn my_task(...) -> T`
- 执行器中 poll 循环检查 Future 的执行状态
  - 内部通过固定写法 `unsafe { Pin::new_unchecked(&mut future) }` 来循环调用，直到处理完毕
- 最终的业务结果：Future::Output

```rs
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};


async fn my_simple_async_task(id: i32) -> i32 {
    println!("Task {} 内部：开始执行异步逻辑。", id);

    // 异步、耗时操作

    println!("Task {} 内部：异步逻辑完成。", id);
    id * 2 // 返回一个结果
}


fn block_on<F: Future>(mut future: F) -> F::Output {
    let mut pinned_future = unsafe { Pin::new_unchecked(&mut future) };
    let waker = dummy_waker();
    let mut context = Context::from_waker(&waker);

    println!("Executor: 开始轮询 Future...");

    loop {
        match pinned_future.as_mut().poll(&mut context) {
            Poll::Pending => {
                // 如果是真正的异步 I/O，线程会在这里去执行其他任务
                println!("Executor: Future 暂时挂起 (Pending)。");
            },

            // Future 完成，返回结果
            Poll::Ready(result) => {
                println!("Executor: Future 完成 (Ready)。");
                return result;
            }
        }
    }
}

fn dummy_waker() -> std::task::Waker {
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}
    unsafe fn clone(_: *const ()) -> std::task::RawWaker { dummy_waker() }

    let raw_waker = std::task::RawWaker::new(
        std::ptr::null(),
        &std::task::RawWakerVTable::new(clone, wake, wake_by_ref, drop)
    );
    unsafe { std::task::Waker::from_raw(raw_waker) }
}


fn main() {
    // ① 调用 async 函数：此时 Future 被创建，但**没有执行**。
    let my_future = my_simple_async_task(42);

    println!("main(): Future 已创建，但尚未被驱动 (惰性)。");

    // ② 手动将 Future 传入执行器，开始执行
    let final_result = block_on(my_future);

    println!("main(): 最终结果是: {}", final_result);
}
```
