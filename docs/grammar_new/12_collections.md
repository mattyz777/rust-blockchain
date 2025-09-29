# vec get

```rs
let v = vec![1, 2, 3];
match v.get(2) {
    Some(val) => println!("The third element is {}", val),
    None => println!("There is no third element."),
}
```

# vec iter

```rs
let mut v = vec![1, 2, 3];
for i in &v {
    *i += 1; // dereference
    println!("{}", i);
}
```
