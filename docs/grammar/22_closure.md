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
    F: Fn(u32) -> u32, // restrict parameter type and returned value type
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

# closure & ownership

```rs
fn call_once<F>(f: F)
where F:FnOnce(), {
  f();
}

fn main() {
    let name1:String = String::from("aaa");

    let closure1 = || {
        println!("Hello1 {}!", name1);
    };

    call_once(closure1);
    closure1();

    // -----------------

    let name2:String = String::from("bbb");
    call_once(||{
        println!("Hello2 {}!", name2);
    });

    println!("{}", name2);

    // ------------------

    let closure2 = || {
        let _moved = name1; // move ownership
        println!("Hello2 {}!", _moved);
    };

    call_once(closure2);
    closure2();  // error since name1 is moved and cannot be consumed again
}
```

# closure String

move keyword is not necessary

```rs
fn main() {
    let s = String::from("hello");

    // --- Borrow (read only)
    let c1 = || println!("{}", s); // just read → borrowed
    c1();
    c1(); // ✅ works

    // --- Move (transfer ownership)
    let c2 = move || {
        let s2 = s; // consume s inside closure
        println!("{}", s2);
    };
    c2(); // ✅ works
    // c2(); // ❌ ERROR: s was moved into closure

    // --- Wrong: move implicitly without move keyword
    let s = String::from("world");
    let c3 = || {
        let s2 = s; // ❌ ERROR: cannot move out of borrowed variable
        println!("{}", s2);
    };
    // c3(); // ❌ fails
}
```

# closure Vec

move keyword is not necessary

```rs
fn main() {
    let mut v = vec![1, 2, 3];

    // --- Borrow (mutable)
    let mut c1 = || v.push(4); // borrows v mutably
    c1();
    c1(); // ✅ works

    // --- Move
    let v = vec![1, 2, 3];
    let mut c2 = move || {
        let mut v2 = v; // v moved into closure
        v2.push(5);
        println!("{:?}", v2);
    };
    c2(); // ✅ works
    // c2(); // ❌ ERROR: v was moved
}
```

# closure Struct

```rs
struct MyStruct {
    data: String,
}

fn main() {
    let s = MyStruct { data: "abc".into() };

    // --- Borrow (read)
    let c1 = || println!("{}", s.data);
    c1();
    c1(); // ✅ works

    // --- Move & consume
    let s = MyStruct { data: "def".into() };
    let c2 = move || {
        let s2 = s; // consume s
        println!("{}", s2.data);
    };
    c2();
    // c2(); // ❌ ERROR: closure can only be called once
}
```

# closure enum

move keyword is not necessary

```rs
enum MyEnum {
    A(String),
    B(i32),
}

fn enum_examples() {
    let e = MyEnum::A("hello".into());

    // --- Borrow
    let c1 = || match &e { // <-------------- &
        MyEnum::A(s) => println!("{}", s),
        MyEnum::B(_) => println!("B"),
    };
    c1();
    c1(); // ✅ works

    // --- Move & consume
    let e = MyEnum::A("world".into());
    let c2 = move || match e { // <-------------- move
        MyEnum::A(s) => println!("{}", s),
        MyEnum::B(_) => println!("B"),
    };
    c2();
    c2(); // ❌ ERROR: closure can only be called once
}
```

# closure primitive

move keyword is not necessary

```rs
fn main() {
    let x = 42;

    // --- Borrow (read)
    let c1 = || println!("{}", x);
    c1();
    c1(); // ✅ works

    // --- Move (also works, Copy)
    let c2 = move || {
        let x2 = x; // consume x
        println!("{}", x2);
    };
    c2();
    c2(); // ✅ works because i32 is Copy
}
```
