use std::thread;

//在线程中使用move闭包
fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {//使用move关键字来强制闭包获得它所需值的所有权
        println!("Here's a vector : {:?}",v);
    });

    handle.join().unwrap();
}
