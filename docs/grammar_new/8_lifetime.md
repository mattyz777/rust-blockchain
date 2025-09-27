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

# lifetime

Lifetime annotations in Rust mostly exist to describe how long references are valid.

# generic lifetime annotation `'a`

- it describes the relationship between the lifetimes of multiple references and how they relate to each other.
- it doesn't change the lifetime of a reference but rather specifies the relationship between them.

# case 1. Both values live long enough → ✅ works

```rust
let s1:String = String::from("hello");
let s2:String = String::from("world");

let result = longest(s1.as_str(), s2.as_str());

// &i32         // a reference
// 'a           // generic lifetime
// &'a i32      // a reference with an explicit lifetime
// &'a mut i32  // a mutable reference with an explicit lifetime
// the lifetime of the return value is the shortest lifetime among the inputs(x, y)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

# case 2. Shorter lifetime but result stays inside the block → ✅ works

```rs

let s1:String = String::from("hello");
{
    let s2:String = String::from("world");

    let result = longest(s1.as_str(), s2.as_str()); // lifetime of result is s2
}

```

# case 3. Shorter lifetime, result escapes the block → ❌ ERROR

```rs

let s1:String = String::from("hello");
let result;
{
    let s2:String = String::from("world");

    result = longest(s1.as_str(), s2.as_str()); // lifetime of result is s2
}
println!("{}", result); // ERROR: result could point to s2

```

# case 4. Return lifetime depends only on x, independent of other inputs

```rs
    let s1: String = String::from("hello");
    let r: &str;
    {
        let s2: String = String::from("world");
        r = longest_string(s1.as_str(), s2.as_str());
    }

    println!("{}", r);

    fn longest_string<'a>(x: &'a str, _: &str) -> &'a str {
        x
    }
```

# case 5. Lifetime of returned value has to be tied to the lifetime of one of the parameters or static lifetime

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

# case 6. Dangling reference due to inner block lifetime

A dangling reference is a reference that points to memory that has already been freed or gone out of scope.

```rs
fn test() {
    let r:&i32;
    {
        let x:i32 = 11;
        r = &x;  // &x ^^ borrowed value does not live long enough
    }
    println!("{}", r);
}
```

# reference type in struct requires lifetime annotation

- Owned fields → no lifetime annotation needed.

```rs
struct ImportantExcerpt<'a> {
    part: &'a str, // reference type
}

let first_sentence = "aa";
let i = ImportantExcerpt {
    part: first_sentence
};
```

# reference type in impl

- lifetime elision: if a method parameter has `&self` or `&mut self`, no need to explicitly write `'a` since Rust automatically tie the return reference lifetime to self.

```rs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
//  fn announce_part(&'a self, announcement: &str) -> &'a str {
    fn announce_part(&self, announcement: &str) -> &str {
        self.part
    }
}

let novel = String::from("Call me Ishmael. Some years ago...");
let excerpt = ImportantExcerpt { part: &novel };
let first = excerpt.announce_part("Here is the first part:");
println!("First part: {}", first);

```

# static lifetime

- `&'static` means the reference exists for the entire duration of the program.
- all string literal has static lifetime which lives in the binary memory.
- `&'static str` in short `&str`

```rs
let s: &'static str = "I have a static lifetime.";
```
