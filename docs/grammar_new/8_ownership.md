# stack & heap

- stack: b() sits on top of a() in the call stack.
- x and y are store on stack inside a()
- "hello" is stored on binary memory (static memory)
- "world" is stored on heap
- z is stored on stack inside b() and z points to "world" on heap

```rs
fn main() {
    fn a() {
        let x:&str = "hello";
        let y:i32 = 22;
        b();
    }

    fn b() {
        let z: String = String::from("world");
    }
}
```

# copy & move

- Types that implement Copy:
  - Primitive scalars: integer, float, bool, char.
  - Tuples/arrays where all elements are Copy.
- Types that don’t implement Copy
  - Heap-allocated containers: String, Vec, Box, HashMap, etc.
  - Tuples/arrays that contain at least one non-Copy element → the whole thing becomes move-only.

```rs
let x:i32 = 5;
let y:i32 = x; // copy

let s1:String = String::from("hello");
let s2:String = s1;  // move

println!("{}, {}", x, s1);
```

# function

- scalar types: copy;
- heap types: move

```rs
let s1 = String::from("hello");
take_ownership(s1); // move
println!("{}", s1); // ❌ compile error: value borrowed after move

let s2 = String::from("hello");
let s21 = take_and_give_back(s2); // move
println!("{}", s21); // ok

let i1 = 4;
make_copy(i1); // copy
println!("{}", i1);

fn take_ownership(s: String) {
  // take ownership of s
}

fn take_and_give_back(s: String) {
  s
}

fn make_copy(x: i32) {
  // copy x
}
```

# NLL

- With NLL (Rust 2018+), the compiler tracks last use of each reference.
- A reference’s "active" period is from the point it’s created until its last usage, not mere creation. However, created are considered as active.
- NLL 判断一个引用是否"活跃"（active），是基于它是否被使用过，而不是是否被创建。

```rs
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;

// last usage of r1/r2: here in println!. This is analyzed by NLL.
println!("{} and {}", r1, r2);

// r1/r2 are "inactive" (NLL)

let r3 = &mut s; // OK now
println!("{}", r3);
```

```rs
let mut s = String::from("hello");
let r1 = &s;        // r1 created →  it becomes "active" (counts as being borrowed) when it is used in println!.
println!("{}", r1); // last usage of r1 → after this, r1 is inactive
let r2 = &mut s;    // allowed because r1 is inactive
```

```rs
let mut s = String::from("hello");
let hello = &s[..5]; // created
let world = &s[..];  // created

// hello and world are "active" (counts as being borrowed) NLL

s.clear();           // ❌ error
```

```rs
let mut s = String::from("hello");
let hello = &s[..5];
println!("{}", hello); // ✅ last usage of hello

let world = &s[..];
println!("{}", world); // ✅ last usage of world

// hello and world are "inactive" (NLL)

s.clear();  // ✅ allowed
```

# mutable reference

- #1 Multiple immutable references are allowed.
- #2 One mutable reference at a time.
- #3 Immutable and mutable references cannot "active" at the same time.
  - This is where Non-Lexical Lifetimes (NLL) come into play.

```rs
// #1
let s1 = String::from("hello");
let s2 = &s1;
let s3 = &s1; // ok

// #2
let mut s1 = String::from("hello");
let s4 = &mut s1;
let s5 = &mut s1; // ❌ compile error

// #3
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;

println!("{}, {}", r1, r2); // ✅ last use of r1, r2 here

let r3 = &mut s;            // ✅ now safe, r1 & r2 are no longer "active"
println!("{}", r3);
```

# println! 参数是 borrowed 引用

为什么报错信息是 "`s1` value borrowed here after move"

```rs
let s1 = test2();
let s3 = s1;
println!("{}", s1);
```

```rs
println!("{}", s1);
// 会被展开成
std::io::_print(format_args!("{}", &s1));
```
