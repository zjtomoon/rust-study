use std::net::UdpSocket;
use std::thread;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3000").expect("error");
    let mut buffer = [0;1024];

    loop {
        let socket_new = socket.try_clone().expect("Unable to new");
        match socket_new.recv_from(&mut buffer) {
            Ok((num_bytes,src_addr)) => {
                thread::spawn(move || {
                    let send_buffer = &mut buffer[..num_bytes];
                    println!("Received from client: {}",std::str::from_utf8(send_buffer).unwrap());
                    let res_str = format!("Received this : {}", String::from_utf8_lossy(send_buffer));
                    socket_new.send_to(res_str.as_bytes(),&src_addr).expect("error");
                });
            }

            Err(err) => {
                println!("Error : {}",err)
            }
        }
    }
}
