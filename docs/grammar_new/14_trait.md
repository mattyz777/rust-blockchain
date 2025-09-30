# rules

- if a trait defines a method without a default implementation, any type implementing that trait must provide an implementation.
- a trait can have any number of default methods

# Basic trait with default method

```rs
trait Greet {
    fn say_hello(&self) {
        println!("Hello, world!");
    }
}
```

# no overriding

```rs
struct Person {
    name: String,
}

impl Greet for Person {} // no need to implement `say_hello`

fn main() {
    let p = Person { name: "Alice".to_string() };
    p.say_hello(); // prints: Hello, world!
}
```

# Overriding the default implementation

```rs
impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello, {}!", self.name); // custom implementation
    }
}

fn main() {
    let p = Person { name: "Alice".to_string() };
    p.say_hello(); // prints: Hello, Alice!
}
```

# default method calling a required method

```rs
trait Animal {
    fn name(&self) -> &str;

    fn speak(&self) {
        println!("{} makes a sound", self.name());
    }
}
```

# trait bounds

T must implement the Printable trait.

```rs
trait Printable {
    fn print(&self);
}

fn show<T: Printable>(item: T) {
    item.print();
}
```

# trait bounds mutliple

T must implement both Printable and PartialOrd.

```rs
fn compare_and_print<T: Printable + PartialOrd>(a: T, b: T) {
    if a > b {
        a.print();
    } else {
        b.print();
    }
}
```

# trait bounds where

Both T and U must implement both Printable and PartialOrd.

```rs
fn compare_and_print<T, U>(a: T, b: U)
where
    T: Printable + PartialOrd,
    U: Printable + PartialOrd,
{
    // ...
}
```
