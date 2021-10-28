use std::collections::HashMap;

fn main() {
    //访问哈希映射中的值
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    for (key,value) in &scores {
        println!("{} :{}",key,value);
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

}
