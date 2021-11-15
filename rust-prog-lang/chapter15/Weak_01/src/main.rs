//使用Weak<T>弱引用来代替Rc<T>避免循环引用，导致内存泄露
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {

}
