# closure

closure = function without name

```rs
let closure = |x:u32| {
    // logic here
    x
};

fn main() {
    println!("{}", closure(1));
}
```

# Fn

- `Fn` trait describes a callable thing(e.g. function or closure)
- `F: Fn(u32) -> u32` means a type `F` that can be called with a `u32` argument and returns a `u32`.
- `f: F ... where` means parameter `f` must match trait bounds `F`.

```rs
fn call_function<F>(f: F, arg: u32) -> u32
where
    F: Fn(u32) -> u32,
{
    f(arg)
}


fn plus_one_fn(x: u32) -> u32 {
    x + 1
}


let plus_one_closure = |x: u32| x * 2;

let result = call_function(plus_one_closure, 5);
println!("Result = {}", result); // 10


let result = call_function(plus_one_fn, 5);
println!("Result = {}", result); // 6
```
