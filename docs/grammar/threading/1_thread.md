# summary

- ```rs
  let t = thread::spawn(move || {
    //
  });
  let t_val = t.join().unwrap();   // wait thread to finish
  ```
- ```rs
  let (tx, rx) = mpsc::channel();   // create a channel
  let t = thread::spawn(move || {
    tx.send(1).unwrap();            // send a value to the main thread
  });
  let t_val = rx.recv().unwrap();   // wait value in main thread (blocks)
  t.join().unwrap();                // wait thread to finish
  ```

# thread

```rs
use std::thread;
use std::time::Duration;

let t = thread::spawn(|| { // start a thread
    println!("Hello from a thread!");
    thread::sleep(Duration::from_secs(1));
    println!("Thread finished.");
});

t.join().unwrap();        // wait thread to finish
```

# thread fn

```rs
use std::thread;
use std::time::Duration;

fn worker(id: i32) {
    println!("Thread {} started", id);
    thread::sleep(Duration::from_secs(1));
    println!("Thread {} finished", id);
}

let t = thread::spawn(|| {
    worker(1);
});

t.join().unwrap();
```

# thread arguments, move & return value

```rs
use std::thread;
use std::time::Duration;

fn worker(name: String, delay: u64) -> String {
    println!("Worker {} started", name);
    thread::sleep(Duration::from_secs(delay));
    println!("Worker {} finished", name);

    format!("Result from worker {}", name)
}

fn main() {
    let name = String::from("A");

    let t = thread::spawn(move || {
        worker(name, 2)  // name is moved here
    });

    // name can't be used here — ownership moved into the thread

    let result = t.join().unwrap();
    println!("Thread returned: {}", result);
}
```

# thread scope

- local variables will be auto referenced in `thread::scope`, so that no need `move` or `'static`
- `thread::scope` is synchronous at the outer level
- All threads inside the scope run concurrently
- data in thread cannot be mutated

```rs
use std::thread;
use std::time::Duration;

fn main() {
    let data = vec![1, 2, 3];
    // let mut data = vec![1, 2, 3]; // data cannot

    // 1. thread::scope is synchronous at the outer level
    thread::scope(|s| {
        s.spawn(|| {   // 2. All threads inside the scope run concurrently
            println!("Thread 1 starting...");
            thread::sleep(Duration::from_secs(2));
            println!("{}", numbers.len());
            println!("Thread 1 done");
        });

        s.spawn(|| {   // 2. All threads inside the scope run concurrently
            println!("Thread 2 starting...");
            thread::sleep(Duration::from_secs(1));
            for n in &numbers {
                println!("{}", n);
            }
            println!("Thread 2 done");
        });

        println!("Inside scope: all threads started"); // 3. all threads are finished, data still valid
    });
    // <---- only reached after all threads inside scope finish
}
```

# mpsc

```rs

use std::sync::mpsc; // 导入 mpsc 模块
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个通道
    // `tx` 是发送端 (Sender)
    // `rx` 是接收端 (Receiver)
    let (tx, rx) = mpsc::channel();

    // 创建一个子线程作为生产者
    let handle = thread::spawn(move || {
        // 子线程拥有 `tx` 的所有权
        let message = String::from("Hello from the spawned thread!");
        println!("等待3秒后发送消息: {}", message);
        thread::sleep(Duration::from_secs(3));
        println!("发送消息: {}", message);
        tx.send(message).unwrap(); // 发送消息
        // `message` 在发送后被移动，不能再使用
    });

    println!("等待消息");
    // 主线程是消费者，拥有 `rx`
    let received = rx.recv().unwrap(); // 阻塞等待，直到收到消息
    println!("收到消息: {}", received);

    println!("等待子线程开始");
    handle.join().unwrap(); // 等待子线程结束
    println!("等待子线程结束");
}
```

```
等待消息
等待3秒后发送消息: Hello from the spawned thread!
发送消息: Hello from the spawned thread!
收到消息: Hello from the spawned thread!
等待子线程开始
等待子线程结束
```

# mpsc multiple messages

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone(); // 创建第二个发送端

    let handle1 = thread::spawn(move || {
        tx.send("来自线程1的消息").unwrap();
    });

    let handle2 = thread::spawn(move || {
        tx2.send("来自线程2的消息").unwrap();
    });

    // 主线程接收两个消息
    for received in rx.iter().take(2) { // 使用迭代器接收
        println!("收到: {}", received);
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
}
```
