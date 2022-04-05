use conrod::backend::glium::glium::{self, Surface};
use conrod::{widget, Positionable, Colorable, Widget};
use conrod::widget::collapsible_area::Ids;

extern crate find_folder;
extern crate conrod;

fn main() {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello Conrod")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    'main: loop {
        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
//     let ui = &mut ui.set_widgets();
//     widget_ids!(struct Ids { text });
//     let ids = Ids::new(ui.widget_id_generator());
//
// // "Hello World!" in the middle of the screen.
//     widget::Text::new("Hello World!")
//         .middle_of(ui.window)
//         .color(conrod::color::WHITE)
//         .font_size(32)
//         .set(ids.text, ui);

    // const WIDTH: i32 = 400;
    // const HEIGHT: i32 = 640;
    // let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // let assets = find_folder::Search::KidsThenParents(3, 5)
    //     .for_folder("assets")
    //     .unwrap();
    // let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    // ui.fonts.insert_from_file(font_path).unwrap();
}
