fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}",x),
        Some(x) => println!("{}",x),
        None => (),
    }
}
//匹配守卫提供的额外条件
//匹配守卫是一个指定于match分支模式之后的if条件，它也必须被满足才能选择此分支。
//匹配守卫用于表达比单独的模式所能允许的更为复杂的情况