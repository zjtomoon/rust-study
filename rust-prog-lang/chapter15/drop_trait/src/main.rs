struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with date `{}`!",self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };

    println!("CustomSmartPointers created");
    //c.drop();
    //不能显式地调用drop
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}
//Rust在实例离开作用域时自动调用了我们编写的drop代码
//因为变量的丢弃顺序与创建顺序相反，所以d在c之前被丢弃。