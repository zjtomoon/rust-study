use screen_gui::Draw;
use screen_gui::{Button,Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        //实际绘制一个选择框的代码
    }
}
fn main() {
    /* let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),

            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok")
            }),
        ],
    };*/

    let screen = Screen {
        components: vec![
            Box::new(String::from("Hi")),
        ],
    };
    screen.run();
}
