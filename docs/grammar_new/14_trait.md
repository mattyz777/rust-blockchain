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

# associated types

```rs
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

// Iterator trait in `std::iter::iterator`
impl Iterator for Counter {
    type Item = u32; // associated type: each iteration yields u32

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new(5);

    while let Some(value) = counter.next() {
        println!("Got: {}", value);
    }

    let counter2 = Counter::new(3);
    for value in counter2 {
        println!("For loop: {}", value);
    }
}
```
