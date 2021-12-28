fn main() {
    let x = Some(5);
    let y = 10;
    //使用匹配守卫来测试与外部变量的相等性
    match x {
        Some(50) => println!("Got 50"),
        //现在第二个匹配分支中的模式不会引入一个覆盖外部y的新变量y
        //这意味着可以在匹配守卫中使用外部变量y
        Some(n) if n == y => println!("Matched,n = {:?}", n),
        _ => println!("Default case,x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

//使用匹配守卫来解决模式中变量覆盖的问题