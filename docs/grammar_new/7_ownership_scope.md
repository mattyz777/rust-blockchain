# scope

- Block scope: A variable declared in an outer block is accessible inside inner blocks.
- Function scope: A variable declared inside a function is not accessible outside that function.

## Block scope

```rs
fn main() {
    let mut x = 10;
    {
        let y = 5;
        println!("x = {}, y = {}", x, y);   // x = 10, y = 5
        x += y;
        let mut x = 2;                      // shadowing
        println!("x = {}, y = {}", x, y);   // x = 2, y = 5
    }
    println!("x = {}", x);                  // x = 15
    // println!("y = {}", y); // ERROR: y is out of scope
}
```

## function scope

```rs
fn main() {
    let x = 10;
    fn test() {
        println!("x = {}", x); // ERROR: x is out of scope
    }
}
```

# Common mistake

When the function ends, s1 is dropped, so the reference would be dangling.

```rs
fn test() -> &str {
    let s1:String = String::from("hello");
    return s1.as_str(); // error
}
```

correct

```rs
// 'static lifetime means the string exists for the entire program.
fn test() -> &'static str {
    "hello"
}
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
