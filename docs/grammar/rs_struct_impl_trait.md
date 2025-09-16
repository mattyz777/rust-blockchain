# struct

```rust
struct Person {
    name: String;  // struct owns the data
}

let p1:Person = Person{
    name: String::from("aa")
}

println!("{} is {} years old", p1.name, p1.age);
```

## struct mutable

When needing to update field value

```rust
let mut p2:Person = Person {
    name: "aa".to_owned()
}

p2.name = "bb"; //  update field value
println!("{} is {} years old", p2.name, p2.age);
```

## tuple

```rust
struct RGBColor(u8, u8, u8);

let red = RGBColor(255, 0, 0);
println!("Red value: {}", red.0);
```

## print struct object

- `#[derive(Debug)]`
- `println!("{:?}", person)`

```rust
#[derive(Debug)]
struct Person {}

println!("{:?}", Person);
```

# impl

- impl block cannot "exist alone".
- It must tie to something concrete(e.g. `struct`, `enum`).

## impl ties to struct

```rust
struct Person { name:String }
impl Person {
    pub fun get_name(&self) -> &str {
        &self.name
    }
}
let p1:Person = Person{name: "aa".to_string()};
p1.get_name();

```

## impl ties to enum

```rust
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
```

## impl ties to trait

```rust
trait Speak {
    fn say_hi(&self);
}

struct Dog;

impl Speak for Dog {
    fn say_hi(&self){
        //
    }
}
```

# method receivers

self, &self, &mute self are method receivers which are only used in methods defined inside an impl block

- `self`, move ownership
- `&self`, immutable borrow (read-only)
- `&mut self`, mutable borrow (modify instance)

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
	// ownership moved
    fn into_another_person(self) -> Person {
        Person {
            name: format!("{} Jr.", self.name),
            age: self.age,
        }
    }

	// Immutable borrow: read-only method
    fn greet(&self) {
        println!("name {} age {}", self.name, self.age);
    }

	// Mutable borrow: modifies the instance
	// let mut <instance>;
	fn update_name(&mut self, name:&str) {
        self.name = name.to_string();
    }
}

let p1 = Person {
	name: "Alice".to_string(),
	age: 30,
};


let p2 = p1.into_another_person(); // Ownership is moved from p1 to p2
p2.greet();                        // "name Alice age 30"
p1.greet();                        // compile error
let mut p2 = p2;                   // p2 can be modified
p2.update_name("Jackson")          //
p2.greet();                        // "name Jackson age 30"
```

## associated function vs method

- method has a receiver parameter (self, &self, or &mut self).
- associated function has no receiver. Looks like a static function tied to a type.

```rust
struct Person {
    name: String,
}

impl Person {
    fn new() -> Self { // associated function
        Person {name: "Alice".into() }
    }

    fn greet(&self) { // method
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    let p:Person = Person::new();
    p.greet();
}
```

## visibility

everything is private by default, only visible within the same mod

```rust
impl Person {
    pub fn into_another_person(self) -> Person {
        ...
    }
}
```

# multiple traits

```rust
// struct
struct Person {
    name: String,
}

// traits
trait Greet {
    fn greet(&self) -> String;
}

trait Farewell {
    fn goodbye(&self) -> String;
}

// impl
impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

impl Farewell for Person {
    fn goodbye(&self) -> String {
        format!("Goodbye, {}!", self.name)
    }
}

// Use multiple traits via generic bounds
fn interact<T: Greet + Farewell>(x: &T) {
    println!("{}", x.greet());
    println!("{}", x.goodbye());
}

// run
let p = Person { name: "Alice".into() };
interact(&p);

//----------------------------------------
// super-trait
//----------------------------------------
trait Communicate: Greet + Farewell {
    fn communicate(&self) {
        println!("{}", self.greet());
        println!("{}", self.goodbye());
    }
}

// impl super-trait
impl Communicate for Person {}

// run
let p = Person { name: "Alice".into() };
p.communicate();
```

# trait bound

sort of restriction on generic types, the generic type must implement specified traits

```rust
// where trait bounds
fn compare_and_print<T>(a: T, b: T)
where T: std::fmt::Debug + PartialOrd,
{
    // ...
}

// inline trait bounds
fn compare_and_print<T: std::fmt::Debug + PartialOrd>(a: T, b: T) {}
```
