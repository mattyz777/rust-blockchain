# String

String is on heap

```rs
let s1:String = String::new();
let s2:String = String::from("hello");
let s3:String = "hello".to_string();
let s4:String = "hello".to_owned();
let s5:String = "hello".into();
let s6:String = "hello".cloned();
```

# String literal

string literals is in static memory not as String on heap

```rs
let slice1:&str = "hello"
// let s1:String = String::from("hello");
let slice2:&str = s1.as_str();
```

# to_string

convert a value to String

```rs
let s1:String = 1.to_string();    // "1"
let s2:String = true.to_string(); // "true"
let s3:String = 'a'.to_string();  // "a"
```

# format macro

convert array/tuple to String

```rs
let s4 = format!("{:?}", [1,2,3]); // "[1, 2, 3]"
let s5 = format!("{:?}", (1,2));   // "(1, 2)"
```
