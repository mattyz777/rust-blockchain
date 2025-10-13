# prerequisites

```rs
let mut v = vec![1, 2, 3];
```

# delete

```rs
let deleted_val = v.remove(1);
```

# position

```rs
let Option<usize> = v.iter().position(|x| *x == id);
let Option<usize> = v.iter().position(|user| user.id == id);
```
