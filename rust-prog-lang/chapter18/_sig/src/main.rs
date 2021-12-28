fn main() {
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello"));

    //if let Some(_s)  = s { //错误，因为s的值仍然会绑定值，它可能会获取值的所有权
    if let Some(_) = s {
        
        println!("found a string");
    }
    println!("{:?}",s);
}

//若希望rust不要警告未使用的变量，可以用下划线作为变量名的开头
//只使用_和使用以下划线开头的名称有些不同，_x仍会将值绑定到变量，而_则完全不会绑定