//使用Weak<T>弱引用来代替Rc<T>避免循环引用，导致内存泄露
use std::rc::{Rc,Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {},weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("leaf parent = {:?}",leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {},weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "leaf strong = {},weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:#?}",leaf.parent.borrow().upgrade());

    println!(
      "leaf strong = {},weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
