# background

- Data can have only one owner at a time.
- Simple borrowing is unsafe across threads because the first thread might drop the data while the second thread is still using a reference to it. This leads to a use-after-free error.

# Arc vs Copy

- if the data is Copy, rust will copy the value.
- if the data is not Copy, rust will move the value. So, `Arc::new` and `Arc::Clone` are needed.

# Arc

ensures shared "ownership" and lifetime, allowing the data to be accessed and modified across multiple threading contexts.

- `let a: Arc<T> = Arc::new(value)`
- `Arc::new` stores value on the heap:
  - Allocates a block of memory on the heap, called the control block.
  - Moves value (the instance of T) into this heap memory.
  - Initializes the reference count (strong_count) to 1.
- The heap memory (the control block) contains two main parts:
  - The reference count (strong_count)
  - The data itself (data: T)
- a lives on the stack as a local variable. Its type is `Arc<T>`.
  - The stack value is a lightweight smart pointer that points to the heap control block.
- `Arc::clone(a)` does the following
  - Copies the stack pointer (cheap, no heap copy)
  - Increments the reference count (strong_count) in the heap control block by 1

```
- `let a: Arc<T> = Arc::new(value)`
- `Arc::new` 在堆上存储 value 数据
  - 在堆上分配一块内存, 即"控制块"（control block）。
  - 把 value（即 T 的实例）移动（move） 到这块内存中
  - 初始化引用计数 `strong_count `（初始化为 1）
- 堆上这块内存即"控制块"包含两个核心部分：
  - 引用计数（strong_count）
  - 数据本身（data: T）
- `a` 存在栈上（栈上的局部变量）；栈上值的类型是 Arc<T>；包含一个指向堆(即"控制块")的指针(轻量级的智能指针)
- Arc::clone()
  - 复制栈上指针，不复制堆上数据
  - 堆上这块内存即"控制块" 中的 引用计数（strong_count）增加 1
```

# Arc readonly int

- counter is i32 which impl Copy trait, when perform `move`, rust will copy the value.
- `let mut counter = 0;` doesn't work even though i32 is Copy in multithreading.

```rs
let counter = 0;

let handle1 = thread::spawn(move || {
    println!("Task1 : {}", counter);
});

let handle2 = thread::spawn(move || {
    println!("Task2: {}", counter);
});

handle1.join().unwrap();
handle2.join().unwrap();
```

# Arc readonly String "wrong"

- name is String which doesn't impl Copy trait. when perform `move`, rust will move the value.

```rust
let name = "hello".to_string();

let h1 = thread::spawn(move || {
    println!("{}", name);
});

let h2 = thread::spawn(move || { // <------------- error due to name is moved
    println!("{}", name);
});
```

# Arc::new & Arc::clone

- `Arc::new(shared_data)`
- `Arc::clone(&shared_data_arc)` used by each thread

```rs
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let shared_data = String::from("Hello, World!");
    let shared_data_arc = Arc::new(shared_data);

    let mut handles = vec![];

    for i in 0..5 {
        let shared_data_arc = Arc::clone(&shared_data_arc); // shadowing
        let handle = tokio::spawn(async move {
            sleep(Duration::from_secs(2)).await;
            println!("Task {}: {}", i, shared_data_arc);
            format!("Task {}: {}", i, shared_data_arc)
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.unwrap();
        println!("Result: {}", result);
    }

    println!("--- program end ---");
}
```

# Mutext

ensures shared data is modified by only one thread at a time.

# Arc Mutex

`Arc<Mutex<T>>` shared mutable state across threads

- Mutext new `Mutex::new(0);`
- Arc new `Arc::new(mutex_object);`
- Arc clone `Arc::clone(&arc_mutex_object);`
- lock & await `let mut num = &arc_mutex_object.lock().await;`
- \* `let result = *num;`

| Step | Code                                          | Description                                |
| ---- | --------------------------------------------- | ------------------------------------------ |
| 1    | `let m = Mutex::new(0);`                      | Create a mutex-protected value             |
| 2    | `let shared = Arc::new(m);`                   | Wrap it in an `Arc` so it can be shared    |
| 3    | `let shared_clone = Arc::clone(&shared);`     | Clone the `Arc` for another thread or task |
| 4    | `let mut num = shared_clone.lock().unwrap();` | Lock it to get **mutable access**          |
| 5    | `*num += 1;`                                  | Modify the inner value                     |
| 6    | `let result = *shared.lock().unwrap();`       | Read the final value                       |

```rs
use tokio::sync::Mutex;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);

        let handle = tokio::spawn(async move {
            let mut num = counter.lock().await;
            *num += 1;
            println!("Task {}: {}", i, *num);
        });  // <---------- lock is auto released when leaving the scope

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let result = *counter.lock().await;
    println!("Result: {}", result); // Result: 5
}
```

# Arc

Only the first-level shared state that must be owned by multiple threads/tasks needs Arc.
Inner values don’t need Arc if they’re already protected (Mutex, Atomic) or cheaply clonable.

Cloning AppState does not clone the contents of the Mutex<Vec<UserDTO>> or the AtomicUsize, only the Arcs are cloned (reference counts incremented).

```rs
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Vec<UserDTO>>>,
    pub id: Arc<AtomicUsize>,
}
```

- db
  - Vec<UserDTO> 是复杂数据，需要互斥访问
  - Mutex 保证线程安全修改
  - Arc 让多个线程共享这个 Mutex 的所有权
- id
  - AtomicUsize 本身线程安全
  - Arc 让多个线程共享这个原子变量的所有权
- AppState
  - AppState 要被克隆给多个线程（比如 Axum 的处理器），所以用 `#[derive(Clone)]`
  - 这里的克隆会 Clone db 和 id, 他们都分别包含了指向堆上 Arc 块的指针。没有克隆数据本身。
  - 每个字段(db, id)内部用了 Arc 来实现共享所有权。
- `let instance = AppState { ... };`
  - instance 是一个栈上的结构体实例, 该结构体包含两个字段 db 和 id
  - db 和 id 都是 Arc<T>类型、固定大小的智能指针
  - db 和 id 的智能指针内部包含指向堆上 Arc 控制块的指针
