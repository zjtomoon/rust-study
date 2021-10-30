pub fn greeting(name: &str) -> String {
    //format!("Hello {} !",name)
    String::from("Hello!")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
 /*   fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
*/
    fn greeting_contains_name() {
        let result = greeting("Carol");
        //assert!(result.contains("Carol"));
        assert!(result.contains("Carol"),
            "Greeting did not contain name,value was `{}` ",result
        );
    }
}
