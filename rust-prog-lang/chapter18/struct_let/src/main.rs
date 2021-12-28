struct Point {
    x: i32,
    y: i32,
}


fn main() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

}

/*
    这段代码创建了变量a和b来匹配结构体p中的x和y字段。
    let Point {x:x,y:y} = p;
    对于匹配结构体字段的模式存在简写：只需要列出结构体字段的名称，则模式创建的变量会有相同的名称。
 */