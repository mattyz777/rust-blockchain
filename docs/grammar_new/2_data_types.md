# types

## scalar types

single value

- Integer: i8 ~ i128 (u8 ~ u128)
- Floating: f32 - f64
- Boolean: bool
- Character: char

| Name    | Type         | Example                        | Description        |
| ------- | ------------ | ------------------------------ | ------------------ |
| `i8`    | Signed int   | `let x: i8 = -10;`             |                    |
| `i16`   | Signed int   | `let x: i16 = 1000;`           |                    |
| `i32`   | Signed int   | `let x: i32 = 100_000;`        | default int        |
| `i64`   | Signed int   | `let x: i64 = 1_000_000;`      |                    |
| `i128`  | Signed int   | `let x: i128 = 1_000_000_000;` |                    |
| `isize` | Signed int   | `let x: isize = 10;`           |                    |
| `f32`   | Float        | `let x: f32 = 3.14;`           |                    |
| `f64`   | Float        | `let x: f64 = 3.141592;`       | default float      |
| `bool`  | Boolean      | `let b: bool = true;`          |                    |
| `char`  | Character    | `let c: char = 'a';`           | `'a'` `'\n'` `'\'` |
| `u8`    | Unsigned int | `let x: u8 = 255;`             |                    |
| `u16`   | Unsigned int | `let x: u16 = 5000;`           |                    |
| `u32`   | Unsigned int | `let x: u32 = 1_000_000;`      |                    |
| `u64`   | Unsigned int | `let x: u64 = 10_000_000;`     |                    |
| `u128`  | Unsigned int | `let x: u128 = 1_000_000_000;` |                    |
| `usize` | Unsigned int | `let x: usize = 10;`           |                    |

## compound types

a group of values

| Name     | Type         | Example                                      | Description              |
| -------- | ------------ | -------------------------------------------- | ------------------------ |
| `vector` | Vec<T>       | `let v: Vec<i32> = vec![1, 2, 3];`           | Growable, heap-allocated |
| `array`  | [T; N]       | `let a: [i32; 3] = [1, 2, 3];`               | Fixed-size list          |
| `tuple`  | (T1, T2, ..) | `let t: (i32, f64, char) = (10, 3.14, 'a');` |                          |
| `struct` | struct       |                                              |                          |
| `enum`   | enum         |                                              |                          |

### tuple

```rs
let tup:(&str, i32) = ("Hello", 42);
let (channel:&str, port:i32) = tup;
println!("channel: {}, port: {}", channel, port
```

# constant

```rs
const PI: f64 = 3.141592;

fn main() {
    println!("PI = {}", PI);
}
```

# bases 进制

```rs
let a: u16 = 33_222u16;      // decimal  (base 10), type u16
let b: u8 = 0b1111_0000u8;   // binary (base 2), type u8
let c: u8 = 0o77u8;          // octal (base 8), type u8
let d: u8 = 0xFFu8;          // hex (base 16), type u8
```

# mutable

- `let mut var = value;`

```rust
let mut x = 10; // mutable variable
println!("x = {}", x);

x = 20; // OK: value can change
println!("x = {}", x);
```

# shadowing

```rs
let x = 10;
let x = x + 1; // 11
```
