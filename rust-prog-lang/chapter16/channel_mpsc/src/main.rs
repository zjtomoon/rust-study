use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx,rx) = mpsc::channel();

    //使用消息传递在线程间转移数据
   /* thread::spawn(move || {
       let val = String::from("hi");
        tx.send(val).unwrap();
        //println!("val is {}",val); //将val发送给通道后再尝试使用它
    });

    let received = rx.recv().unwrap();
    println!("Got :{}",received);*/

    //发送多个值并观察接收者的等待过程
    //通过克隆发送者创建多个生产者
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            //tx.send(val).unwrap();
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //通过克隆发送者创建多个生产者
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}",received);
    }
}
