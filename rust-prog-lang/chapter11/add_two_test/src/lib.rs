pub fn add_two(a: i32) -> i32 {
    a + 2
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    fn it_adds_two() {
        assert_eq!(4,add_two(2));
    }
}
