# slice type

```rs
// Slice of an array
let arr = [10, 20, 30, 40];
let slice: &[i32] = &arr[1..3];

// Slice of a vector
let v = vec![10, 20, 30, 40, 50];
let s: &[i32] = &v[1..4];
println!("Slice: {:?}", s);

// Mutable slice
let mut arr = [1, 2, 3];
let slice: &mut [i32] = &mut arr[..];
slice[0] = 42;

// Heap-allocated slice (Box<[T]>)
let boxed: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
```

# type alias
