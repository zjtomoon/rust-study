pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
/*        if value < 1|| value > 100 {
            panic!("Guess must be between 1 and 100,got {}.",value);
        }
        Guess {
            value
        }*/
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1,got {}.",value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100,got {}.",value);
        }
        Guess {
            value
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;//用于在模块树中指明条目的路径。
                 //因为tests是一个内部模块，所以我们必须将外部模块中的代码导入内部模块的作用域中
    #[test]
    /*fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }*/
   #[should_panic(expected="Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
