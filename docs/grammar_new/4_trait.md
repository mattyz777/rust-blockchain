# Summary

| Trait            | Summary                                                |
| ---------------- | ------------------------------------------------------ |
| ToString         | convert a value to String via `.to_string()`           |
| Display          | `{}` in `println!` , enables `.to_string()`            |
| #[derive(Debug)] | `{:?}` or `{:#?}` in `println!`                        |
| From<T>          | convert to another type with `.into()` or `From::from` |
| ToOwned          | turn a borrowed reference into an owned value          |
| Copy             | Indicates duplicable, requires Copy                    |

# ToString

- convert a value to String via `.to_string()`
- integer/float/char/bool implement `ToString` by default.

```rs
let s1:String = 1.to_string();    // "1"
let s2:String = true.to_string(); // "true"
let s3:String = 'a'.to_string();  // "a"
```

# Display

- used for `{}` in `println!`.
- It enables calling `.to_string()` on custom struct which implements Display.
  ```rs
  println!("{}", person);             // uses Display
  println!("{}", person.to_string()); // via ToString auto-impl
  ```
- takes &self → borrows, doesn’t consume → you can still use the struct afterwards.

## Display impl

```rs
struct Person {
    id: u32,
}

use std::fmt;
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person: {}", self.id) // implememtation
    }
}

let person = Person { id: 42 };

// println! calls Display::fmt directly
println!("{}", person); // Person: 42
```

## Display + ToString

The standard library provides a blanket implementation of ToString for all Display types:

```rs
/**
 * in core::string::ToString
 * for any type T that implements fmt::Display, also implement ToString.
 */
impl<T: fmt::Display + ?Sized> ToString for T {
    #[inline]
    fn to_string(&self) -> String {
        let mut buf = String::new();
        fmt::write(&mut buf, format_args!("{}", self))
            .expect("a Display implementation returned an error unexpectedly");
        buf
    }
}
```

```rs
// ToString::to_string → calls Display::fmt
let s: String = person.to_string();
println!("{}", s); // Person: 42
```

# `#[derive(Debug)]`

- Used for `{:?}` or `{:#?}` in `println!`

```rs
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p = Person { name: "Alice".to_string(), age: 30 };
    println!("{:?}", p);  // Debug: for developers
    println!("{:#?}", p); // Pretty Debug
}
```

# From<T>

- Convert one type into another.
- `.into()` is just a convenience that calls `From::from`.
- move, the original value cannot be used afterwards.

```rs
struct Person {
    id: u32,
}

struct UserId(String);

// implement From<Person> for UserId
impl From<Person> for UserId {
    fn from(p: Person) -> Self {
        UserId(format!("Person: {}", p.id))
    }
}

fn main() {
    let p1 = Person { id: 42 };
    let p2 = Person { id: 42 };

    let uid1 = UserId::from(p1); // p1 is moved → cannot use p1 after this
    let uid2: UserId = p2.into(); // p2 is moved → cannot use p2 after this

    println!("{}", uid1.0); // Person: 42
}
```

# ToOwned

- Turn a borrowed reference (&T) into an owned value

```rs
// &str → string
let s: &str = "hello";
let owned: String = s.to_owned(); // allocate a new String "hello"

// &[T] → Vec<T>
let arr: &[i32] = &[1, 2, 3];
let v: Vec<i32> = arr.to_owned(); // allocate a new Vec<i32> [1,2,3]

```

# Copy

- Indicates a type’s value can be duplicated.
- Values are fixed-size and stored on the stack.
  - integer/float/char/bool;
  - tuples or arrays where all elements are Copy
  - String, Vec, Box, HashMap, etc. cannot derive Copy. `#[derive(Copy)]`
- All Copy types must also be Clone

```rs
let x: i32 = 10;
let y = x; // copied, x is still usable

let tup: (i32, bool) = (42, true);
let tup2 = tup; // copied because all elements are Copy
```

```rs
#[derive(Debug, Copy, Clone)] // <---------- Copy + Clone; all Copy types must also be Clone
enum IP {
    V4 = 4,
    V6 = 6,
}
```

# Clone

When a struct implements the Clone trait using `#[derive(Clone)]`, instance can be duplicated with .clone():

```rs
#[derive(Clone)]
struct Person {}

let p = Person {};
let p_1 = p.clone();
```
