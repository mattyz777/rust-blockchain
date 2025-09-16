# box

`Box<T>` puts a value of type T on the heap and gives you ownership of it.

```rust
let b: Box<i32> = Box::new(42); // allocate 42 on the heap
println!("b = {}", b);           // prints "b = 42"

/**
 * 0u8: A literal value 0 of type u8 (an 8-bit unsigned integer).
 * 1_000_000: length of the array. 1_000_000 == 1000000
 * An array of 1,000,000 elements, each initialized to 0u8.
*/
let data = Box::new([0u8; 1_000_000]);
println!("Length: {}", data.len());
/**
 * That array would live on the stack, which is risky because the stack is much smaller than the heap (often only a few MB).
 */
let data = [0u8; 1_000_000];


```
