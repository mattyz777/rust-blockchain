# Mutext<T>

- Mutual Exclusion（互斥
- 只允许一个线程访问该数据。
- `	.lock()`

```rs
use std::sync::Mutex;

let data = Mutex::new(0); // 创建一个保护整数 0 的 Mutex

{
    let mut num = data.lock().unwrap(); // 尝试获取锁
    // 成功获取锁，现在可以安全地修改数据
    *num += 1; // 内部数据变为 1
    // num（MutexGuard）离开作用域，锁自动释放
}

// 此时，另一个线程可以再次获取锁
```

# RwLock<T>

- 读写锁, 读共享、写互斥
- `.read()` & `.write()`

```rs
use std::sync::RwLock;

let data = RwLock::new(vec![1, 2, 3]);

// --- 多个线程可以同时读取 ---
{
    let read_guard1 = data.read().unwrap();
    let read_guard2 = data.read().unwrap();
    println!("数据内容: {:?}", *read_guard1);
    // read_guard1 和 read_guard2 离开作用域，读锁释放
}

// --- 写入必须独占 ---
{
    let mut write_guard = data.write().unwrap(); // 获取写锁
    write_guard.push(4); // 修改数据
    // write_guard 离开作用域，写锁释放
}
```
