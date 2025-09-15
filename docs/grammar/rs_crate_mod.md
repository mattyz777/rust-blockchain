# crate

A crate is the smallest unit of compilation in Rust.

When running cargo build, Cargo compiles one or more crates. Each crate produces a single binary (.exe, .out, etc.) or a library (so, .dll, etc.)

# two main types

- binary crate

  - By convention, the entry point is src/main.rs.
  - Must have a main function

- library crate
  - By convention, the entry point is src/lib.rs.
  - No main function

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

# import module

```
proj_crm/
├── Cargo.toml             <-- workspace
├── crm_lib/
│   ├── Cargo.toml         <-- [package] name = "crm_lib"
│   └── src/
│       ├── lib.rs         <-- pub mod contact;
│       └── contact/
│           ├── contact.rs <-- use crate::contact::tag::Tag;
│           ├── mod.rs     <-- pub mod tag;
│           └── tag.rs     <-- pub struct Tag;
└── crm_app/
    ├── Cargo.toml         <-- depends on crm_lib
    └── src/main.rs        <-- use crm_lib::contact::tag::Tag;
```

```rust
// Within the same crate, "crate::"
use crate::contact::tag::Tag;


// From a different crates, "crm_lib"
use crm_lib::contact::tag::Tag;
```
