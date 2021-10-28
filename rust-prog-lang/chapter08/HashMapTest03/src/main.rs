use std::collections::HashMap;

fn main() {
    //更新哈希映射
    //覆盖旧值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Blue"),25);
    println!("{:?}",scores);

    //只在键没有对应值时插入数据
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"),10);

    scores1.entry(String::from("Yellow")).or_insert(50);
    scores1.entry(String::from("Blue")).or_insert(50);
    //or_insert方法被定义为返回一个Entry键所指向值的可变引用，
    // 假如这个值不存在，就将参数作为新值插入哈希映射中，并把这个新值的可变引用返回
    println!("{:?}",scores1);

    //基于旧值来更新值
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}",map);
}
