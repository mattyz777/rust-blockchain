# array

```rust

```

# vector

```rust

// init
let mut v1: Vec<i32> = Vec::new();    // created with new
let mut v2: Vec<i32> = vec![1, 2, 3]; // created with vec! (marco)

// len
println!("len: {}", v.len());

// is_empty
println!("is_empty: {}", v.is_empty());

// contains
println!("contains: {}", v.contains(&1));

// sort
println!("sorted: {}", v.sort());

// reverse
println!("reverse: {}", v.reverse());

// access
let first = v[0];
let maybe:Option<i32> = v.get(1);

// push
v.push(1);

// pop
let n1:Option<i32> = v.pop();

// interator

// tags.iter() produces an iterator over &T rather than T.
// so the param type is &T
let r:Vec<&T> = tags.iter() // fake code

// iterate via for-loop
let tag_names: Vec<&str>;
for tag in tags.iter() {
    tag_names.push(tag.name.as_str()) if tag.is_active();
}

// iterate via map
let tag_names: Vec<&str> = tags.iter().map(|t| t.name.as_str()).collect();
```

# hasmap

# tuple
