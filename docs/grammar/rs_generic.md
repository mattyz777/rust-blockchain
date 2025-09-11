```rust
impl<T> Option<T> {
    fn is_some_and(&self) -> bool {
        matches!(self, Some(_))
    }
}


impl<T: ToString> Vec<T> {
    fn join_to_string(&self) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
    }
}
```
