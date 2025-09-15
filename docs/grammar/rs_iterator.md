# design

- `iter()` is a method defined on `Vec<T>`
- `iter()` returns a concrete type (usually written as `std::slice::Iter<'_, T>`) which implements the Iterator trait.
- Each call to next() on it yields an `Option<&T>` — a reference to the element.
- Avoids copying or moving elements from the vector unless you explicitly want to.

```rust
let v = vec![1, 2, 3];
// creates an iterator over &i32
// iter type is "std::slice::Iter<'_, T>"
let mut iter = v.iter();

assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), None); // no more elements
```

# Iterator trait API

```rust
let v = vec![1, 2, 3];

// map transform;
// v.iter() → Iterator<Item = &i32> (each element is a reference like &1, &2, &3)
// map(|x| x * x) → transforms each &i32 to an i32 (dereference happens implicitly with *x)
// collect() → consumes the iterator and gathers results into a new Vec<i32>
let v_squared: Vec<i32> = v.iter().map(|x| x * x).collect();

v.iter().for_each(|x| println!("{}", x));

// v.iter() → Iterator<Item = &i32>
// each x in v.iter() is a reference to i32
// find passes a reference to the item into the closure → closure receives &&i32
// Returns Some(&2) for the first even number, or None if no match
let first_even:Option<&i32> = v.iter().find(|&&x| x % 2 == 0);

let even_squared: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).collect();
```

# iterator

| Method              | Return type  | Notes                                | Collected with `collect()`  |
| ------------------- | ------------ | ------------------------------------ | --------------------------- |
| `v[i]`              | `&T`         | Panics if out-of-bounds              | N/A                         |
| `v.get(i)`          | `Option<&T>` | Safe, returns None if missing        | N/A                         |
| `v.iter()`          | `&T`         | Iterates over references             | `Vec<&T>`                   |
| `v.iter().cloned()` | `T`          | Iterates and clones elements         | `Vec<T>`                    |
| `v.iter_mut()`      | `&mut T`     | Iterates for modification            | `Vec<&mut T>` (rarely used) |
| `v.into_iter()`     | `T`          | Moves ownership, consumes the vector | `Vec<T>`                    |
