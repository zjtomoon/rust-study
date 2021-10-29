struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point {
        x: 5,
        y: 10
    };

    let float = Point {
        x: 1.0,
        y: 4.0
    };

/*     let wont_work = Point {
        x: 5,
        y: 4.0
    } */
    //字段x和y必须是相同的类型，因为它们拥有相同的泛型T

}
