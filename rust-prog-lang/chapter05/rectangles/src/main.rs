/*fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1,height1)
    )
}

fn area(width: u32,height: u32) -> u32 {
    width * height
}
*/


//使用元组重构代码
/*fn main() {
   let rect1 = (30,50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/
//使用结构体重构代码
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
   /*  fn square(size: u32) -> Rectangle {
        Rectangle { width: size,height: size }
    } */
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self,other:Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {width:30,height:50};

    println!(
        "The area of the rectangle is {} square pixels.",
        Rectangle::area(&rect1)
    );
}

/* fn area(rectangle: Rectangle) ->u32 {
    rectangle.width * rectangle.height
} */