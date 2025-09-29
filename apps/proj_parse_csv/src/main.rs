/**
 * # add dependency
 * cargo add clap --features=derive
 * cargo add csv
 * cargo add serde --features derive
 * 
 * # usage
 * rcli csv -i input.csv -o output.json --header -d ','
 * 
 * # run
 * cd apps/proj_parse_csv
 * 
 * # cargo help cmd
 * cargo run --help
 *
 * # app help cmd due to clap
 * cargo run -- help
 * 
 * # subcmd help cmd due to clap
 * cargo run -- csv -i input.csv
 * cargo run -- csv --input input.csv
 * 
 * --------------------------------------
 * #[arg(short, long, help = "Input file path", value_parser = verify_input_file, default_value = "input.csv")]
 * 
 * 
 * 
 * rust builds two crates: lib and binary in current folder structure
 * `use crate::opts::Opts;` => the compiler looks for a module named opts in the binary crate (which only contains main.rs) ⇒ Not found ⇒ error.
 * ----------------------
 * apps/
 *   proj_parse_csv/
 *     src/
 *       lib.rs
 *       main.rs
 *     Cargo.toml
 * ----------------------
 * 
 * 
 * Package: rcli (from Cargo.toml)
 * ├── Library crate (root: src/lib.rs)
 * │     pub mod opts;
 * │
 * └── Binary crate  (root: src/main.rs)
 *       use rcli::opts;   // OK
 *       use crate::opts;  // ❌ (no opts in this crate)
 * 
 **/
// use clap::Parser;
// use csv::Reader;
// use crate::opts::{Opts, SubCommand, Record}; // not working
// use rcli::opts::{Opts, SubCommand, Record};


fn main() {
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
}
