# vec print

```rs
let v = vec![1, 2, 3];
println!("{:?}", v);
```

# vec get

```rs
let v = vec![1, 2, 3];
match v.get(2) { // return Option<&i32>
    Some(val) => println!("The third element is {}", val),
    None => println!("There is no third element."),
}
```

# vec iter for immutable

- `let mut v` -> `for i in &v` -> `i: &i32`
- `let mut v` -> `for i in &mut v` -> `i: &mut i32` -> `*i += 1`

```rs
let mut v = vec![1, 2, 3];
for i in &v {           // i: &i32
    println!("{}", i);  // [2, 3, 4]
}


let mut v = vec![1, 2, 3];
for i in &mut v {       // i: &mut i32
    *i += 1;            // dereference
    println!("{}", i);  // [2, 3, 4]
}
```

# vec iter next

- `v.iter()` -> `&` -> `iter.next()`
- `v.into_iter()` -> `iter.next()`

```rs
#[test]
fn test_vec_iterator() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));

    let mut v2_iter = v1.into_iter();
    assert_eq!(v2_iter.next(), Some(1));
    assert_eq!(v2_iter.next(), Some(2));
}
```

# vec iter().sum()

```rs
let v = vec![1, 2, 3];
let total = v.iter().sum();
```

# vec iter().map

```rs
let v1: Vec<_> = v1.iter().map(|x| x + 1).collect();
println!("{:?}", v1); // [2, 3, 4]
```
