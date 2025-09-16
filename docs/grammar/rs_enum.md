# 3 layers

```rust
#[derive(Debug)]
enum AppState<T> {
    Loading {
        progress: f32, // field with type
    },
    Loaded {
        screen: Screen<T>, // field with type
    },
    Error {
        code: u16, // field with type
        message: String, // field with type
    },
}

#[derive(Debug)]
enum Screen<T> {
    Home,
    Details(Detail<T>),
}

#[derive(Debug)]
struct Detail<T> {
    id: u32,
    data: T,
}


// 1️⃣ A "Loading" state
let s1: AppState<()> = AppState::Loading { progress: 0.42 };

// 2️⃣ A "Loaded -> Details" state
let s2: AppState<String> = AppState::Loaded {
    screen: Screen::Details(Detail {
        id: 7,
        data: "User profile".to_string(),
    }),
};

// 3️⃣ An "Error" state
let s3: AppState<()> = AppState::Error {
    code: 500,
    message: "Server down".into(),
};

println!("s1 = {:?}", s1);
println!("s2 = {:?}", s2);
println!("s3 = {:?}", s3);

```
