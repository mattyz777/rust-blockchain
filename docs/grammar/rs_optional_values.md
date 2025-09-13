# Optional Values

```rust
fn main() {
    let mut v = vec![String::from("hello"), String::from("world")];

    // 1.1 Direct indexing, panics if out-of-bounds
    let s:String = v[0];    // WRONG. this tries to move ownership out of the vector via indexing, which is not allowed

    // 1.2 indexing
    let n1:&String = &m[0]; //panics if out-of-bounds
    println!("{}", n1); // prints "hello"

    // 2. get optionally
    let b: Option<&i32> = v.get(5); // OK. Returns None if the index is out-of-bounds. Safe alternative to `v[i]`.
    match b {
        Some(v) => println!("Safe get: {}", v),
        None => println!("Out of bounds"),
    }

    // 3. Iterate and clone
    //    v.iter() gives Iterator<Item = &String>
    //    .cloned() turns &String into String by calling Clone
    //    .collect() gathers Strings into a new Vec<String>
    let v_cloned: Vec<String> = v.iter().cloned().collect();
    println!("v_cloned: {:?}", v_cloned);

    // 4. iter_mut(): mutable borrow for modification
    for s in v.iter_mut() {
        s.push_str("!!!");
    }
    println!("v after iter_mut: {:?}", v);

    // 5. into_iter(): consumes the vector and moves ownership
    //    v is no longer usable
    let v2 = v.into_iter().collect::<Vec<String>>();
    println!("v2: {:?}", v2);

}

```

# None default value

```rust
let s1: Option<String> = None;
let value = s1.unwrap_or("default value".to_string());
println!("{}", value); // prints: default value
```

# handle optional values

```rust
let m:Vec<String> = vec![String::from("hello"), String::from("world")];

// 1 indexing
let n1:&String = &m[0]; //panics if out-of-bounds
println!("{}", n1); // prints "hello"

// 2.1 get with match
let n2:Option<&String> = m.get(0);
match n2 {
    Some(value) => println!("First element: {}", value),
    None => println!("Out of bounds"),
}

// 2.2 get with if let
if let Some(value) = m.get(0) {
    println!("First element: {}", value);
}

// 2.3  map with default value
let n3 = m.get(0).map(|v| v.clone()).unwrap_or("default value".to_string());


```
