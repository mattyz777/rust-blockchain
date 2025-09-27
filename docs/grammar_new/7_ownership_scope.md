# println! 参数是 borrowed 引用

为什么报错信息是 "`s1` value borrowed here after move"

```rs
let s1 = test2();
let s3 = s1;
println!("{}", s1);
```

```rs
println!("{}", s1);
// 会被展开成
std::io::_print(format_args!("{}", &s1));
```
