enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}
fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
/*
当 Rust 编译这些代码的时候，它会进行单态化。编译器会读取传递给 Option<T> 的值并发
现有两种 Option<T> ：一个对应 i32 另一个对应 f64 。为此，它会将泛型定义 Option<T>
展开为 Option_i32 和 Option_f64 ，接着将泛型定义替换为这两个具体的定义。

 */