# Dereferencing

Dereferencing is the process of accessing the value that a reference points to.

```rust
/**
 * x is stored on the stack,
 * x contains the actual value 5.
 */
let x = 5;

/**
 * y is stored on the stack,
 * y contains the memory address of x.
 */
let y = &x;

/**
 * Stack:
 * [x] = 5
 * [y] = address_of_x
 */

// Dereference y to get 10
println!("y points to: {}", *y);

// mut
let mut n = 10;
let p = &mut n;
*p += 5;
println!("n is now: {}", n);

```

# rules

- Read scenario → auto-deref
- Write scenario → explicit `*`

# Implicitly dereferences

Implicit dereferencing is that the compiler automatically inserts dereference operations `*`.

- Method calls
- Field access
- Operators (like +, -)
- Function arguments, if types implement Deref

## method call

Auto-(de)referencing happens when you call a method with ..

```rust
let s = String::from("hello");
let len = s.len(); // s.len() → (&s).len() automatically.
```

## Field access

```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 3, y: 4 };
let r = &p; // r is a reference to a Point

println!("{}", r.x); // `r.x` → (*r).x
```

## Operators

```rust
let x = 5;
let y = &x;

println!("{}", y + 1); // y + 1 → *y + 1  automatically.
```

## Function arguments

```rust
fn print_len(s: &str) {
    println!("{}", s.len());
}

let string = String::from("hello");
print_len(&string); // &String → &str automatically
```

# When must written

When assigning / mutating through a mutable reference:

```rust
let mut n = 10;
let r = &mut n;
*r += 1;       // explicit deref required to change the value
```
