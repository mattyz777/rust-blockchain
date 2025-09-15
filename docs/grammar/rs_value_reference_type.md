# #[derive(Debug)]

```rs
/**
 * Return a copy of the value with ownership
 * Commonly used to return primitive types which are small and cheap to copy
 */
pub fn age(&self) -> u8 {
    self.age
}

/**
 * Reference to the age field of the value that self points to.
 * sort of &(self.age)
 */
pub fn age(&self) -> &u8 {
    &self.age
}
```
