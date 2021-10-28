use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //匹配不同的错误
    let f = File::open("hello.text");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.text") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem : {:?}",e),
            },
            other_error => panic!("There was a problem opening the file : {:?}",other_error),
        },
    };
}
