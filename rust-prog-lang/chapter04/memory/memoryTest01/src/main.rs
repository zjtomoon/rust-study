fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{},world!",s1);//为了避免复制分配的内存，Rust在这种场景下会简单地将s1废弃
    println!("{},world!",s2);
}

/*
    Rust永远不会自动地创建数据的深度拷贝。因此在Rust中，任何自动的赋值都可以被视为高效的。
*/