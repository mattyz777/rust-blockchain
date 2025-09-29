# Enum without data

```rs
#[derive(Debug)]
enum IP {
    V4,
    V6,
}

let ip = IP::V4;
println!("ip {:?}", ip);
```

# Enum with parameters

```rs
#[derive(Debug)]
enum IP {
    V4(String),
    V6(String),
}

let ip = IP::V4(String::from("127.0.0.1"));
println!("ip {:?}", ip);
```

# Enum as Copy parameters (move issue)

```rs
#[derive(Debug)]
enum IP {
    V4, // zero-sized types; stored on stack
    V6,
}

let ip = IP::V4;

fn test(ip:IP) {
    match ip {
        IP::V4 => println!("Routing IPv4: {:?}", ip),
        IP::V6 => println!("Routing IPv6: {:?}", ip),
    }
}

test(ip);
println!("ip {:?}", ip); // ❌ error, ip was moved since by default function call are moved.
```

# Enum with String parameters requires Clone

```rs
#[derive(Debug, Clone)] // <------------ move by default
enum IP {
    V4(String),
    V6(String),
}

let ip = IP::V4(String::from("127.0.0.1"));
println!("ip {:?}", ip);

fn test(ip:IP) {
    match ip {
        IP::V4(addr) => println!("Routing IPv4: {}", addr),
        IP::V6(addr) => println!("Routing IPv6: {}", addr),
    }
}

test(ip.clone());
println!("ip {:?}", ip);
```

# enum with String(move) parameters

```rs
#[derive(Debug, Copy, Clone)]
enum IP {
    V4(String),
    V6(String),
}

let ip = IP::V4(String::from("127.0.0.1"));
println!("ip {:?}", ip);

fn test(ip:IP) {
    match ip {
        IP::V4(addr) => println!("Routing IPv4: {}", addr),
        IP::V6(addr) => println!("Routing IPv6: {}", addr),
    }
}

test(ip);
println!("ip {:?}", ip);
```
