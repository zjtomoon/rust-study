//使用Rc<T>和RefCell<T>结合使用来实现一个拥有多重所有权的可变数据
//Rc<T>允许多个所有哲持有同一数据，但只能提供针对数据的不可变访问。如果我们在Rc<T>内存储了RefCell<T>，
// 那么就可以定义出拥有多个所有者且能够进行修改的值
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>,Rc<List>),
    Nil,
}

use crate::List::{Cons,Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value),Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));

    let c = Cons(Rc::new(RefCell::new(10)),Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {:#?}",a);
    println!("b after = {:#?}",b);
    println!("c after = {:#?}",c);
}
