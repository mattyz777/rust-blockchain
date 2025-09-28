# for

## for range

```rs
for item in 0..10 {
    // [0,9)
}
```

## for range start end

```rs
let start = 0;
let end = 100;

for item in start..end {
    println!("{}", item);
}
```

## for array

```rs
let arr = [1, 2, 3];
for item in arr() {
    //
}
```

## for vector

```rs
let vec = vec![1, 2, 3];
for item in &vec() {
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
