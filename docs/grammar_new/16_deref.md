# types

| Type    | What you have          | What you must do    | Why                          |
| ------- | ---------------------- | ------------------- | ---------------------------- |
| `&i32`  | reference to primitive | must write `*x`     | no auto-deref for comparison |
| `&User` | reference to struct    | can write `user.id` | field access auto-derefs     |

# Struct field access auto-deref

```rs
struct User {
    id: usize,
}

let users = vec![User { id: 1 }, User { id: 2 }];
let id = 2;

let pos = users.iter().position(|user| user.id == id);
```

# Primitive types require explicit deref

integer, float, char, boolean

```rs
let v = vec![1, 2, 3];
let id = 2;

let pos = v.iter().position(|x| *x == id);
```
