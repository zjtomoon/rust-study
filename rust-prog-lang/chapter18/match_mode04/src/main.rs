fn main() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("someting else"),
    }

    let y = 'c';
    match y {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

/*
   通过..=匹配值的范围

   ..=语法允许匹配一个闭区间范围内的值
*/
