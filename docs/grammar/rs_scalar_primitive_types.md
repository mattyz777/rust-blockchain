# scalar types

- default types are `i32` & `f64`
- integer, float, boolean, char
- A scalar value is stored wherever its variable is stored.

| Type  | Bits           | Bytes          | Default | Example                |
| ----- | -------------- | -------------- | ------- | ---------------------- |
| i8    | 8              | 1              |         | `let x: i8 = -42;`     |
| i16   | 16             | 2              |         | `let x: i16 = -42;`    |
| i32   | 32             | 4              | yes     | `let x: i32 = -42;`    |
| i64   | 64             | 8              |         | `let x: i64 = -42;`    |
| i128  | 128            | 16             |         | `let x: i128 = -42;`   |
| isize | pointer size\* | pointer size\* |         | `let x: isize = -42;`  |
| f32   | 32             | 4              |         | `let x: f32 = 3.14;`   |
| f64   | 64             | 8              | yes     | `let x: f64 = 3.1415;` |
| bool  | 8              | 1              |         | `let x: bool = true;`  |
| char  | 32             | 4              |         | `let x: char = 'A';`   |
| u8    | 8              | 1              |         | `let x: u8 = 42;`      |
| u16   | 16             | 2              |         | `let x: u16 = 42;`     |
| u32   | 32             | 4              |         | `let x: u32 = 42;`     |
| u64   | 64             | 8              |         | `let x: u64 = 42;`     |
| u128  | 128            | 16             |         | `let x: u128 = 42;`    |
| usize | pointer size\* | pointer size\* |         | `let x: usize = 42;`   |

# primitive Types

It includes all `scalar types` and `compound types`.

- Compound types: `Tuple`, `Array`

# precision

- use 3rd depdencies named `rust_decimal`, `rust_decimal_macros`
- all operations are done with `Decimal`
- two ways to create a `Decimal`
  - `Decimal::from()` associated function
  - `dec!()` macro

```rust
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn main() {
    let a: Decimal = dec!(2.99);
    let b: Decimal = Decimal::from(3);

    // Arithmetic [əˈrɪθmətɪk] operations
    let c1: Decimal = a + b;
    let c2: Decimal = a - b;
    let c3: Decimal = a * b;
    let c4: Decimal = a / b;

    println!("c1: {}, c2: {}, c3: {}, c4: {}", c1, c2, c3, c4);
}
```
