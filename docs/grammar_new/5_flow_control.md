# if / else if / else

```rs
fn test() {
    let a = 1;

    if a == 1 {
        println!("a is 1");
    } else if a == 2 {
        println!("a is 2");
    } else {
        println!("a is not 1 or 2");
    }
}
```

# if let

```rs
let m = if true { 1 } else { 2 };
```

# match

## match numeric

```rs
let x = 1;
match x {
    1 => println!("x is 1 "),
    2|3|5|7|11 => println!("x = {} is prime", x),
    _ => println!("x is not 1 or 2"),
}
```

## match enum

```rs
enum Color {
    Red,
    Green,
    Blue,
}

let color:Color;
match color {
    Color::Red => println!("color is red"),
    Color::Green => println!("color is green"),
    Color::Blue => println!("color is blue"),
    _ => println!("color is not red or green or blue"),
}
```

## match tuple

```rs
let x = (1, 2);

match x {
    (1, 2) => println!("x is (1, 2)"),
    (1, _) => println!("x is (1, _)"),
    (_, 2) => println!("x is (_, 2)"),
    _ => println!("x is not (1, 2)"),
}
```

## match guard

```rs
let x = 1;
match x {
    x if x < 0 => println!("x is negative"),
    x if x > 0 => println!("x is positive"),
    _ => println!("x is zero"),
}
```
