use std::thread;
use std::time::Duration;

//使用spawn创建新线程
fn main() {
    let handle = thread::spawn(|| {
       for i in 1..10 {
           println!("hi number {} from the spawned thread!",i);
           thread::sleep(Duration::from_millis(1000));
       }
    });
    //handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!",i);
        thread::sleep(Duration::from_millis(1000));
    }

    handle.join().unwrap(); //保存thread::spawn的JoinHandle来保证新线程能够执行完毕。
    //在线程句柄上强调join函数会阻塞当前线程，直到句柄代表的线程结束。
}
