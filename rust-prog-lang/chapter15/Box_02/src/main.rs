fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5,x);
    assert_eq!(5,*y);

    let x1 = 5;
    let y1 = Box::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);
}
