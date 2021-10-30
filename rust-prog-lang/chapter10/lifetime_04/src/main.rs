use std::fmt::Display;

fn longest_with_an_announcement<'a,T>(x: &'a str,y: &'a str,ann: T) -> &'a str where T: Display{
    println!("Announcement! {}",ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
    ann 的类型是泛型 T ，它可以被放入任何实现了 where 从句中指定的
    Display trait 的类型。这个额外的参数会在函数比较字符串 slice 的长度之前被打印出来，这
    也就是为什么 Display trait bound 是必须的。因为生命周期也是泛型，所以生命周期参数
    'a 和泛型类型参数 T 都位于函数名后的同一尖括号列表
 */
fn main() {

}
