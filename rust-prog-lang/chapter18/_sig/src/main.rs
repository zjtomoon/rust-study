fn main() {
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello"));

    //if let Some(_s)  = s { //错误，因为s的值仍然会移动进_s，并阻止我们再次使用s
    if let Some(_) = s { //能够无错编译，因为s没有被移动进_
        
        println!("found a string");
    }
    println!("{:?}",s);
}

//若希望rust不要警告未使用的变量，可以用下划线作为变量名的开头
//只使用_和使用以下划线开头的名称有些不同，_x仍会将值绑定到变量，而_则完全不会绑定