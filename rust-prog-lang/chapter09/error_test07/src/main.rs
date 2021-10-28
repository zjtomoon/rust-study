use std::io;
use std::fs;

fn main() {

}


//最简短的写法
fn read_username_from_file() -> Result<String,io::Error> {
    fs::read_to_string("hello.txt");
}
