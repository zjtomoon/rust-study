use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//通过实现Deref trait来将类型视作引用
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0 //意味着deref会返回一个指向值的引用，进而允许调用者通过*运算符访问值
    }
}

fn hello(name: &str) {
    println!("Hello,{}",name);
}

//自定义智能指针
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
