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
