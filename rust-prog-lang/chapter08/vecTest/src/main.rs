fn main() {
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];

    println!("The third element is {} ",third);

    match v.get(2) {
        Some(third) => println!("The Third element is {} ",third),
        None => println!("There is no third element."),
    }

    let v1 = vec![100,32,57];
    for i in &v1 {
        println!("{}",i);
    }

    println!("==========");

    let mut v2 = vec![100,32,57];
    for j in &mut v2 {
        *j += 50;
        println!("{}",j)
    }
}
