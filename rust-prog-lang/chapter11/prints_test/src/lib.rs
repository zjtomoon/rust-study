/*
    默认情况下，当测试通过时，Rust 的测试库会截获打印到标准输出的所有内容。比如在测试
    中调用了 println! 而测试通过了，我们将不会在终端看到 println! 的输出：只会看到说
    明测试通过的提示行。如果测试失败了，则会看到所有标准输出和其他错误信息。
 */
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}",a);
    10
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10,value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5,value);
    }
}
