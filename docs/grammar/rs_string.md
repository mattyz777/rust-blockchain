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

```rust
let s1:String = String::new();
let s2:String = String::from("aa"); // convert string literal to String
let s3:String = s1.clone(); // String clone, A new String is created on the heap.
let s4:String = "aa".to_string();
let s5:String = "aa".to_owned();
let s6:String = "aa".into(); // &str into, A new String is created on the heap.
let s7:String = format!("{}", "aa"); // format macro allocates a new String on the heap
```

# to_string

```rs
2.to_string();  // number
true.to_string(); // boolean
"aa".to_string(); // string literal

```

# string literal

```rust
let s1:String = String::from("bb");

let slice1:&str = "aa";
let slice2:&str = &s1;    //Borrow as &str, slice2[2..4] works
let slice3:&String = &s1; //Borrow as &str, slice2[2..4] NOT work, it points to the whole String.
let slice4: &str = s1.as_str();
```

# how to create String

- `to_string` requires trait `Display`
- `to_owned` requires trait `Clone`
- `into` requires trait `From<T>`

```rust
let s1 = String::from("aa");
let s2 = "aa".to_string();
let s3 = "aa".to_owned();
let s3: String = "aa".into(); // &str to String
let s4: String = s1.clone(); // String clone, s4 has the ownership
let s5 = format!("hi {}", "aa");
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

- text content is `variable` with `println!("{}")`
- stack address is `&variable` with `println!("{:p}")`
- heap address is `variable.as_ptr()` with `println!("{:p}")`

```rust
let n2: &str = "aa";
println!("{}", n2);  // Uses Display implementation for &str to print the text content of the slice
println!("{:p}", n2); // address of the string data on heap
println!("{:p}", &n2); // address of the reference variable on stack
```

# ownership

```rust
{
	let owner = "hello".to_string(); // `owner` owns a heap-allocated String

	let mut s = String::from("hi");  // mutable String
	s.push_str(", world!");
} // Scope ends here
  // both `s` and `owner` go out of scope
  // Rust automatically calls `drop` for both
  // Heap memory is freed safely
```

# String concatenation

## String clone

```rust
// both n1 and n2 have the ownership
let n1:String = "hello".to_string();
let n2 = n1.clone() + " world"; // hello world
println!("{}", n2);
```
