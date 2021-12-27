fn main() {
    let favourite_color:Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8,_> = "34".parse();
    if let Some(color) = favourite_color {
        println!("Using your favourite color,{},as the background",&color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("using purple as the background color");
        } else {
            println!("using orange as the background color");
        }
    } else {
        println!("using blue as the background color");
    }
}
