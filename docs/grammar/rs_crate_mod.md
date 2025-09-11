# crate

A crate is the smallest unit of compilation in Rust.

When running cargo build, Cargo compiles one or more crates. Each crate produces a single binary (.exe, .out, etc.) or a library (so, .dll, etc.)

# two main types

- binary crate

  - Must have a main function
  - By convention, the entry point is src/main.rs.

- library crate
  - Don't have a main function.
  - By convention, the entry point is src/lib.rs.

# creating rust project

- `cargo init` initializes a project in an existing directory in current folder.
- `cargo init <proj_name>` initializes a project in an existing directory in new folder .
- `cargo new <proj_name>` creates a new directory.

both init and new have two params `--lib` and `--bin`

```
demo_2018
├── Cargo.toml
└── src
    └── main.rs      <- bin, crate root
```

```
demo_2018
├── Cargo.toml
└── src
    └── lib.rs      <- lib, crate root
```

# cargo package

- A Cargo package is Rust project
- A package contains one or more crates (binary or library)

# crate root

- The crate root is a single source file (main.rs or lib.rs) that the compiler starts from.
- `crate::` is `src/main.rc` in example below.

```
demo_2018
├── Cargo.toml
└── src
    └── main.rs      <- bin, crate root
```

# module (mod)

A module inside a crate used to split(manage) code into files and subdirectories.

- `mod x;` searches for the module in the current directory of the file.
  - It first looks for a file named x.rs in the same directory.
  - If x.rs does not exist, it looks for a subdirectory named x containing mod.rs.

```
demo_2024
├── Cargo.toml
└── src
    ├── main.rs
    └── foo.rs
    └── foo
        ├── a.rs
        └── b.rs
```

## main.rs

```rust
mod foo;

fn main() {
    crate::foo::a::hello(); //  absolute path
    foo::b::world();        //  relative path
}
```

## foo.rs

```rust
pub mod a;
pub mod b;
```

## a.rs

```rust
pub fn hello() {
	println!("2018::foo::a");
}
```

## b.rs

```rust
pub fn hello() {
	println!("2018::foo::b");
}
```
