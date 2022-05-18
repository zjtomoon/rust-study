fn main() {

    // 不可变引用
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.",s1,len);

    // 可变引用
    let mut s = String::from("hello");
    
    change(&mut s);

    println!("{}",s);
    println!("{}",&s);

    // 可变引用同时只能存在一个
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}，{}",r1,r2);

    // 可变引用与不可变引用不能同时存在

    // 悬垂引用
    let reference_to_nothing = dangle();
    println!("{}",reference_to_nothing)
}


fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(",world");
}

//悬垂引用也叫做悬垂指针，意思为指针指向某个值后，
//这个值被释放掉了，而指针仍然存在，
//其指向的内存可能不存在任何值或已被其它变量重新使用。
//在 Rust 中编译器可以确保引用永远也不会变成悬垂状态：
//当你拥有一些数据的引用，编译器可以确保数据不会在其引用之前被释放，要想释放数据，必须先停止其引用的使用。 
fn dangle() -> String {
    let s = String::from("hello");
    s
}

//next:复合类型