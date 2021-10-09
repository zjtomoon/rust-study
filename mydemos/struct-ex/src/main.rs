#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}
/*
    关联函数
    构造函数
    实例方法
*/
impl Rectangle {
    fn new(width:i32,height:i32) -> Self {
        Rectangle {
            width,
            height
        }
    }
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

/* impl Rectangle {
    fn area(width:i32,height:i32) ->i32 {
        width * height
    }
} */

fn main() {
/*     let demo = Rectangle {
        width: 3,
        height: 4,
    }; */

    let demo = Rectangle::new(3, 4);

    println!("{}",demo.height);
    println!("{:?}",demo);

    
    let num = demo.area();
    //let num = Rectangle::area(3, 4);
    println!("{}",num);
}
