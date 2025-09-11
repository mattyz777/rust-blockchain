# string literal

String literal is

## core

- immutable
- borrowed view
- don't take ownership.

## explaination

- The actual bytes are stored in binary’s static memory (read-only).
- The variable s is just a reference (&str) pointing to that static memory.
- Sometimes called string slice
- has type of `&'static str`

## example

```rust
let name:&str = "aa";
```

# String

- Heap-allocated → data stored on the heap
- growable → append, push.
- Owned → responsible for freeing memory when dropped (goes out of scope).

# how to create string literal

```rust
let s1:String = String::from("bb"); // data preparation

let slice1:&str = "aa";
let slice2:&str = &s1;    //Borrow as &str, slice2[2..4] works
let slice3:&String = &s1; //Borrow as &str, slice2[2..4] NOT work, it points to the whole String.
let slice4: &str = s1.as_str();
```

# how to create String

```rust
let s1 = String::from("aa");
let s2 = "aa".to_string();
let s3 = "aa".to_owned();
let s3: String = "aa".into(); // Needs type annotation (or inference).
let s4 = format!("hi {}", "aa");
```

# String comparisons are by value(content) not by memory address

```rust
let s1:String = "hello".to_string();
let s2:String = "hello".to_string();
let s3:String = String::from("hello");
let s4:String = String::from("hello");
// all of them are true.
println!("s1 equals s2 {} equals s3 {} equals s4 {}", s1==s2, s2==s3, s3==s4)
```

# address

no matter String or &str or &String

- stack address is `&variable`
- heap address is `variable.as_ptr()`

```rust
let slice1:&str = "aa";
let s1:String = String::from("aa");

// get addresses for String
println!("stack address of s variable is {:p}", &s1);
println!("heap  address of `aa` data content is {:p}", s1.as_ptr());

// get addresses for string literal
println!("stack address: {:p}", &slice1);
println!("heap  address: {:p}", slice1.as_ptr());
```

# ownership

```rust
{
	let owner = "hello".to_string(); // `owner` owns a heap-allocated String

	let mut s = String::from("hi");  // mutable String
	s.push_str(", world!");
} // Scope ends here
  // `s` and `owner` go out of scope
  // Rust automatically calls `drop` for both
  // Heap memory is freed safely
```
