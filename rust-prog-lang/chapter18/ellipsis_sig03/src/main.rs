fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // (.., second, ..) => { //有歧义
        //     println!("Some numbers: {}", second);
        // },
        (_) => ()
    }
}
// .. 的使用必须时无歧义的，如果期望匹配和忽略的值是不明确的，Rust会报错