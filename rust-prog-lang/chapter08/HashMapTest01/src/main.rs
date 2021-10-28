use std::collections::HashMap;

fn main() {
    //创建一个新的哈希映射
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    println!("{:#?} ",scores);

    let teams = vec![String::from("Blue"),String::from("Yellow")];
    let initial_scores = vec![10,50];
    let scores2: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    //哈希映射与所有权
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let  mut map = HashMap::new();
    map.insert(field_name,field_value);
    // 一旦键值对被插入，其所有权就会转移给哈希映射

}
