use gtk::prelude::*;
use gtk::{Application,ApplicationWindow,Button};
fn main() {
    // let app = Application::builder()
    // .application_id("org.example.HelloWorld")
    // .build();

    // app.connect_activate(|app| {
    //     let win = ApplicationWindow::builder()
    //     .application(app)
    //     .default_width(320)
    //     .default_height(200)
    //     .build();

    //     win.show_all();
    // });

    // app.run();

    let application = Application::builder()
    .application_id("com.example.FirstGtkApp")
    .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
        .application(app)
        .title("First GTK Program")
        .default_width(350)
        .default_height(70)
        .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.add(&button);
        window.show_all();
    });

    application.run();
}

// 参考： https://www.gtk.org/docs/language-bindings/rust/
// https://lib.rs/crates/gtk