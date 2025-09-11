# comments

```rust
/**
 * -> ///
 * -> //
 * -> /* */
 *
 */
```

# function

- function definition
- implicit return (no semicolon)
- main function

```rust
fn add2(x: i32, y: i32) -> i32 {
    // Implicit return (no semicolon)
    x + y
}

fn main() {

}
```

# variables

- variables in method parameter and return type `(<name>: <type>, ...) -> <type>`
- variables declaration `let v:type = value`

```rust
fn add(x: i32, y: i32) -> i32 {
    let x: i32 = 1;
}
```

# mutable / immutable

```rust
let mut mutable = 1;
mutable = 7;   // correct

let immutable = 2;
immutable = 6; // error: cannot assign twice to immutable variable
```

# shadowing

```rust
let z = 1;      // z = 1, type i32, immutable
let z = z + 1;  // shadows the previous z, now z = 2, still immutable
let z = "hi";   // shadows again, now z = "hi", type &str
```

# String

- String is stored as a UTF-8 encoded in terms of `Vec<u8>`

# String literal

`&str` String literals, immutable, borrowed from the binary’s memory (static memory)

```rust
let x: &str = "hi";  // x is `'static` lifetime,
                     // x borrows the data "aaa" from the binary’s memory (static memory).
println!("{}", x);   // hi
```

# String

`String` heap-allocated, growable, owner of its data

```rust
let mut s: String = String::from("hi");
s.push_str(" there");
println!("{}", s);       // "hi there"
```

- `"...". to_string()` create a new String

# Converting `String` & `&str`

- convert &str to String
  - Rust creates a new owned and heap-allocated String.
  - `let s1: String = "hello".to_string();`
  - `let s2: String = String::from("hello");`
- convert String to &str (borrowing)
  - `let s: String = String::from("hello");`
  - `let slice: &str = &s;`

# String slices(&str)

```rust
let s: String = String::from("hello world"); // s owns the heap-allocated string
let slice: &str = &s[0..5];  // slice is a &str, borrowed reference into `s`'s data
```

# ownership

- Every value in Rust has an owner.
- When the owner goes out of scope, Rust automatically drops the value (frees memory).

```rust
// Owned integer (i32)
let x: i32 = 10;      // x owns the integer

// Owned vector (Vec<i32>)
let v: Vec<i32> = vec![1, 2, 3];  // v owns the heap-allocated vector

// Owned String
let s: String = String::from("hello"); // s owns the heap-allocated string
```

# borrowed values

- References (&T or &mut T) borrow ownership temporarily.
- The variable that owns the value remains responsible for freeing it.

```rust
let s: String = String::from("hello");
let r: &String = &s;  // r borrows s, does not own


fn print_string(s: &str) {
    println!("{}", s);                      // prints hello
}

fn main() {
    let s: String = String::from("hello");  // s owns the string
    print_string(&s);                       // pass a reference (borrow)
    println!("{}", s);                      // ✅ still valid, s owns it, prints hello
}
```

# ownership transfer

```rust
fn take_string(s: String) {
    println!("{}", s);    // prints hello
}

fn main() {
    let s: String = String::from("hello");
    take_string(s);       // ownership moved to the function
    // println!("{}", s); // error: s no longer owns the value
}
```

# structure

# types

| Type | bytes |
| ---- | ----- |
| i32  | 4     |
| f64  | 8     |

# macro !

```rust
println!("{}", x)
```

# array

Fixed size, Homogeneous type(all elements must be the same type), Usually stored on the stack

```rust
// allocates 4 × i32 = 16 bytes on the stack.
let four_ints:[in32; 4] = [1,2,3,4]
```

# vector

- a growable, heap-allocated collection
- `vec!` is a macro

```rust
let mut vector: Vec<i32> = vec![1,2,3,4];
vector.push(5); // [1,2,3,4,5]

let slice1:&[i32] = &vector; // reference to the underlying array of elements.
let slice2:&Vec<i32> = &vector; // reference to the entire Vec object

println!("{:?} {:?}", vector, slice1); // [1,2,3,4,5],[1,2,3,4,5]
```

# tuples

Fixed size set of value with possible different types

```rust
let x:(i32, &str, f64) = (1, "hi", 2.2);
let (a, b, c) = x;
println!("{} {} {}", a, b, c);
println!("{}", x.1); // indexing
```

# struct

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin: Point = Point {x:0, y:0};
println!("({}, {})", origin.x, origin.y);

// Tuple struct (a.k.a. unnamed struct)
struct Point2(i32, i32);
let origin2 = Point2(0, 0);
println!("({}, {})", origin2.0, origin2.1);
```

# enum

```rust
// base enum
enum Direction {
    Left,
    Right,
}
let left = Direction::Left;


// enum with fields
enum OptionalI32 {
    AnI32(i32),
    Nothing,
}

let two:OptionalI32 = OptinoalI32::AnI32(2);
let nothing = OptionalI32::Nothing;


```

# generic

```rust
sturcut Foo<T> { bar: T}

enum Optional<T> {
    SomeVal(T),
    NoVal,
}

impl<T> Foo<T> {
    fn bar(&self) -> &T {
        &self.bar
    }

    fn into_bar(self) -> T {
        self.bar
    }

    fn bar_mut(&mut self) -> &mut T {
        &mut self.bar
    }
}
```

# trait

trait is as interface in java

```rust
trait Frobnicate<T> {
    fn frobnicate(self) -> Option<T>;
}

impl<T> Frobnicate<T> for Foo<T> {
    fn frobnicate(self) -> Option<T> {
        Some(self.bar)
    }
}
```

# Pattern matching

```rust
// enum example
let foo = OptionalI32::AnI32(1);

match foo {
    OptionalI32::AnI32(n) => println!("it's an i32: {}", n),
    OptionalI32::Nothing => println!("it's nothing!"),
}

// struct example
struct FooBar { x: i32, y: OptionalI32 }

let bar = FooBar { x: 15, y: OptionalI32::AnI32(32) };

match bar {
    FooBar { x: 0, y: OptionalI32::AnI32(0) } =>
        println!("The numbers are zero!"),
    FooBar { x: n, y: OptionalI32::AnI32(m) } if n == m =>
        println!("The numbers are the same"),
    FooBar { x: n, y: OptionalI32::AnI32(m) } =>
        println!("Different numbers: {} {}", n, m),
    FooBar { x: _, y: OptionalI32::Nothing } =>
        println!("The second number is Nothing!"),
}

```

# for

```rust
for i in array {
    println!("{}", i);
}
```

# range

```rust
for i in iu32..10 {
    println!("{}", i);
}
```

# if

```rust
if i == 1 {
    println!("{}", i);
} else {
    println!("{}", i);
}
```

# if as expression

```rust
let value = if true {
    "a"
} else {
    "b"
}
```

# while

```rust
while 1 == 1 {
    // ...
    break;
}
```

# infinite loop

```rust
loop {
    println!("");
    break;
}
```

# memory safty & pointers

```rust
let mut mine: Box<i32> = Box::new(3);
*mine = 5;
```
