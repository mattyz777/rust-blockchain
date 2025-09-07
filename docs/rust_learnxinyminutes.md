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
let mut mutable = 1; // mutable = 7;   correct
let immutable = 2;   // immutable = 6; error: cannot assign twice to immutable variable
```

# shadowing

```rust
let z = 1;      // z = 1, type i32, immutable
let z = z + 1;  // shadows the previous z, now z = 2, still immutable
let z = "hi";   // shadows again, now z = "hi", type &str
```

# String

- String is stored as a UTF-8 encoded in terms of `Vec<u8>`
- `&str` String literals, immutable
  - ```rust
    let x: &str = "hi";
    println!("{}", x);   // hi
    ```
- `String` heap-allocated, growable
  - ```rust
    let mut s: String = String::from("hi"); // let mut x:String =
    s.push_str(" there");    // append
    println!("{}", s);       // "hi there"
    ```
- `"...". to_string()` create a new String
- convert &str to String
  - Rust creates a new owned and heap-allocated String.
  - `let s1: String = "hello".to_string();`
  - `let s2: String = String::from("hello");`
- convert String to &str (borrowing)
  - `let s: String = String::from("hello");`
  - `let slice: &str = &s;`

# String slices(&str)

```rust
let s: String = String::from("hello world");
let slice: &str = &s[0..5];  // slice of a String
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


fn print_string(s: &String) {
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

# types

- i32
- f64
-
