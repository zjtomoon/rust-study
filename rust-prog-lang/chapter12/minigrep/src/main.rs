extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {

    //将命令行参数收集到一个动态数组中并打印出来
    //let args: Vec<String> = env::args().collect();

    //println!("{:?}",args);
    //let (query,filename) = parse_config(&args);
    //let config = Config::new(&args);
    //let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
    let config = Config::new(env::args()).unwrap_or_else( |err| { //env::args的返回值就是一个迭代器
        //println!("Problem parsing arguments: {}",err);
        eprintln!("Problem parsing arguments: {}",err);
        process::exit(1);
    });
    //let query = &args[1];
    //let filename = &args[2];

    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);

    if let Err(e) = minigrep::run(config) {
        //println!("Application error : {}",e);
        eprintln!("Application error: {}",e);

        process::exit(1);
    }

    // let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    //
    // println!("With text:\n {}",contents);
    // run(config);

}

/*fn run(config: Config) {

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n {}",contents);
}*/


/*fn parse_config(args: &[String]) -> (&str,&str) {
    let query = &args[1];
    let filename = &args[2];

    (query,filename)
}*/


/*fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {
        query,
        filename
    }
}*/