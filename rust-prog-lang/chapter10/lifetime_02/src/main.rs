//结构体定义中的生命周期标注
struct ImportantExcerpt<'a> {
    part: &'a str
}
fn main() {
    let novel = String::from("Call me Ishmael.Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.' ");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}
