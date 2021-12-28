struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 1, y: 2, z: 4 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}
//通过使用..来忽略Point中除x以外的字段
