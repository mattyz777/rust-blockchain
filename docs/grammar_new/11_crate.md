# types of crate

- `src/main.rs` runnable crate
- `src/lib.rs` library crate

# default crates

- If your project has only `src/main.rs` → Cargo builds a binary crate.
- If your project has only `src/lib.rs` → Cargo builds a library crate.
- If your project has both `src/main.rs` and `src/lib.rs` → Cargo builds two crates:
  - a binary crate (from main.rs)
  - a library crate (from lib.rs)
  - even though nothing in Cargo.toml where is used for defining crates.
  - a runnable crate will be created

# crate rules

- one package could have 0-1 library crate
- one package could have 0-many binary crates

# library crate

## library crate - creation

```
carge new --lib restaurant
```

## library crate - structure

```rs
crate
└── front_of_house
├── hosting
│ ├── add_to_waitlist
│ └── seat_at_table
└── serving
├── take_order
├── serve_order
└── take_payment
```

## library crate - src/lib.rs

```rs
mod front_of_house;
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}
```

## library crate - invoke library crate

```toml
# Cargo.toml
[dependencies]

```

## library crate - use absolute path

```rs
crate::front_of_house::hosting::add_to_waitlist();
```

## library crate - use short

```rs
use crate::front_of_house::hosting;
hosting::add_to_waitlist();
```
