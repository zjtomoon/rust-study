fn main() {
    //生成其他迭代器的方法
    let v1: Vec<i32> = vec![1,2,3];

    //v1.iter().map(|x| x + 1);
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    //调用map方法创建新迭代器，接着再调用collect方法将其消耗掉并得到一个动态数组

    assert_eq!(v2,vec![2,3,4]);

}
