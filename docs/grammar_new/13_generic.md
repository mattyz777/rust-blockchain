# Generic & Lifetime

- Returning references from a struct method → lifetime required.
- Returning owned values → lifetime not required.

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

impl <T, U> Point<T, U> {
    fn mixup_reference<'a, 'b, V, W>(&'a self, other: &'b Point<V, W>) -> Point<&'a T, &'b W> {
        Point {
            x: &self.x,  // <-- reference, not ownership
            y: &other.y, // <-- reference, not ownership
        }
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }

}

let p1 = Point { x: 5, y: 10.4 };
let p2 = Point { x: "Hello", y: 'c' };
let p3 = p1.mixup_reference(&p2);

let p4 = Point { x: 15, y: 110.4 };
let p5 = Point { x: "World", y: 'd' };
let p6 = p4.mixup(p5);

println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c
println!("p2.x = {}, p2.y = {}", p2.x, p2.y); // p2.x = Hello, p2.y = d
println!("p2.x = {}, p2.y = {}", p6.x, p6.y); // p2.x = 15, p2.y = d
```
