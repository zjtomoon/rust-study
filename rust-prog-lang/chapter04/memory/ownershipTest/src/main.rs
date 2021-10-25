fn main() {
    let s = String::from("hello");

    take_ownership(s);

    let x = 5;
    makes_copy(x);
}


fn take_ownership(some_string: String) {
    println!("{}",some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}",some_integer);
}

/*
    拥有Copy这种trait的类型：
    1、所以的整数类型，诸如u32
    2、仅拥有两种值的布尔类型:bool
    3、字符类型：char
    4、所有的浮点类型，诸如f64
    5、如果元组包含的所有字段的类型都是Copy的，那么这个元组也是Copy的。
*/