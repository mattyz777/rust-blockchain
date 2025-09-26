# for

```rs
for i in 0..10 {
    //
}
```

# while

```rs
let x = 10;
while x < 10 {
    //
}
```

# break

```rs
let x = 10;
while x < 10 {
    if x == 5 {
        break;
    }
}
```

# loop

```rs
loop {
    //
}
```

# break loop return value

```rs
fn main() {
    let mut x = 10;

    let result = loop {
        if x == 5 {
            break x + 3; // returns 8
        }
        x -= 1;
    };

    println!("{}", result); // 8
}
```
