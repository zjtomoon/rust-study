fn print_coordinates(&(x,y):&(i32,i32)) {
    println!("Current location: ({},{})",x,y);
}

fn main() {
    let point = (3,5);
    print_coordinates(&point);
}
// 一个在参数中结构元组的函数