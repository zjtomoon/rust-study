//
// use std::env;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     println!("size = {}", args.len());
//     for argument in args {
//         println!("{}", argument);
//     }
// }

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("size = {}", args.len());

    let a = &args[1];
    println!("a = {}", a);
    let b = &args[2];
    println!("b = {}", b);
}