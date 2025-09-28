# ToString

- System trait used to convert a value with any type into a String.
- integer/float/char/bool implement `ToString` by default.
- Usually implemented via the `Display` trait.

```rs
// system trait
pub trait ToString {
    fn to_string(&self) -> String;
}
```

# to_string

convert a value to String

```rs
let s1:String = 1.to_string();    // "1"
let s2:String = true.to_string(); // "true"
let s3:String = 'a'.to_string();  // "a"
```

# Display

- Implementing Display defines how type is formatted when using "{}".
- It enables calling `.to_string()` on custom struct which implements Display.
  ```rs
  println!("{}", person);             // uses Display
  println!("{}", person.to_string()); // via ToString auto-impl
  ```
- takes &self → borrows, doesn’t consume → you can still use the struct afterwards.

## Display for

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
 * system trait
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

# From<T>

- Define how to convert one type into another.
- Consumes the input value (takes ownership), so the original value cannot be used afterwards.
- `.into()` is just a convenience that calls `From::from`.

```rs
// system trait
pub trait From<T> {
    fn from(value: T) -> Self;
}

// system trait
impl<T, U> Into<T> for U
where
    T: From<U>,
{
    fn into(self) -> T {
        T::from(self)
    }
}
```

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

Turn a borrowed reference (&T) into an owned value

```rs
pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
}
```

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
- Values are fixed-size and stored entirely on the stack. integer/float/char/bool; tuples or arrays where all elements are Copy

```rs
let x: i32 = 10;
let y = x; // copied, x is still usable

let tup: (i32, bool) = (42, true);
let tup2 = tup; // copied because all elements are Copy
```
