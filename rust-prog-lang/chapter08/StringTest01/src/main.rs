fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}",s1);
    println!("s2 is {}",s2);

    let s3 = String::from("hello,");
    let s4 = String::from("world");
    let s5 = s3 + &s4;
    println!("s5 is {}",s5);

    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");

    let s = format!("{}-{}-{}",s6,s7,s8);
    println!("s is {}",s);
}
