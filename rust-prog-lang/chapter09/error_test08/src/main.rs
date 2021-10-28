use std::error::Error;
use std::fs::File;

fn main() -> Result<(),Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(());
}

/*
       panic! 宏代表一个程序无法处理的状态，并停止执行而不是使用无效或不正确的值继续处理。Rust 类型系统的 Result 枚
       举代表操作可能会在一种可以恢复的情况下失败。可以使用 Result 来告诉代码调用者他需要处理潜在的成功或失败。
       在适当的场景使用 panic! 和 Result 将会使你的代码在面对不可避免的错误时显得更加可靠。
 */