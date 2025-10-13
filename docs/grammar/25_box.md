# smart pointer

```rs
let x = Box::new(5); // value 5 stored on heap, box (pointer) on stack
```

# Why Box<T>

- put data on the heap
- size is unknown at compile time
- recursive types (e.g., linked lists, trees).
- indirection

# example

- `b` is an owning smart pointer (a Box<i32>) stored on the stack.

```rs
let b = Box::new(10);       // heap allocation
println!("b = {}", b);      // auto-deref, no need to write *b
println!("value = {}", *b); // explicit deref also works
```

# recursive types

```rs
// 错误！这会导致无限大小
// enum List {
//     Cons(i32, List), // Cons 包含另一个 List，无限嵌套
//     Nil,
// }

// 正确！使用 Box 来间接持有
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // recursive type stored in a Box
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

# pointer and auto-deref

```rs
let a = 10;
let b1: &i32 = &a;
let b2: &i32 = &a;
let b3: Box<i32> = Box::new(a);
let b4: Box<i32> = Box::new(a);

println!("{} {} {} {}",
    *b1,  // explicit deref of b1
    b2,   // auto deref of b2
    b3,   // auto deref of box b3
    *b4,  // explicit deref of box b4
);
```
