use std::io;
use std::fs::File;
use std::io::Read;
//传播错误
fn main() {

}

//使用match将错误返回给调用者的函数
fn read_username_from_file() -> Result<String,io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}