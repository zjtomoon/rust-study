fn main() {
    //通过将代码提取为函数来减少重复工作
    let number_list = vec![34,50,25,100,65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}",largest);
}
