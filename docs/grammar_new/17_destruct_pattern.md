# destruct pattern

- the lowest level parameter in method must be a field without a type.

# struct -> struct

```rs
struct A {
    value: i32,
}

struct B {
    a: A,
}

// way one
fn test(B { a: A { value } }: B) {
    println!("{}", value);
}

// way two
fn test(B { a }: B) {
    println!("{}", a.value);
}
```

# struct -> tuple

```rs
struct A {
    value: i32,
}

struct B(pub A);

fn test(B(a): B) {
    println!("{}", a.value);
}
```
