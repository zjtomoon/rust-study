fn main() {
    let s = String::from("hello");

    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    // 
    let s1 = gives_ownership();
    let s2 = String::from("hello"); //s2 转移所有权后已被销毁
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {},s3 = {}",s1,s3)
}

// 函数传值与返回
// 将值传递给函数，一样会发生 移动 或者 复制，就跟 let 语句一样，下面的代码展示了所有权、作用域的规则：

fn takes_ownership(some_string: String) {
    println!("{}",some_string);
}

fn makes_copy(some_integer:i32) {
    println!("{}",some_integer);
}

// 同样的，函数返回值也有所有权，例如:
fn gives_ownership() -> String{
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string:String) -> String {
    a_string
}