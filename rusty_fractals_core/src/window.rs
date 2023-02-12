use ColorDepth::Rgb8;
use fltk::{frame::Frame, prelude::*, window::Window};
use fltk::app::{App, event_key};
use fltk::enums::{ColorDepth, Event, Key};
use fltk::image::RgbImage;
use fltk::window::DoubleWindow;

pub fn show(fractal_name: &'static str, domain_image: image::RgbImage, result_image: &image::RgbImage) {
    let width = domain_image.width() as i32;
    let height = domain_image.height() as i32;

    let app = App::default();
    let mut window_domain = make_window(fractal_name, app, width, height, domain_image);
    let mut window_result = make_window(fractal_name, app, width, height, result_image.clone());

    window_domain.show();
    window_result.show();
    app.run().unwrap();
}

fn make_window(fractal_name: &'static str, app: App, width: i32, height: i32, image: image::RgbImage) -> DoubleWindow {
    let image_rgb = RgbImage::new(&image.into_raw(), width, height, Rgb8).unwrap();

    let mut window = Window::default().with_label(fractal_name).with_size(width, height).center_screen();
    let mut frame = Frame::new(0, 0, width, height, "");
    frame.set_image(Some(image_rgb));
    window.add(&frame);
    window.handle(move |_, event| match event {
        Event::KeyDown => {
            if event_key() == Key::Escape {
                println!("exit");
                app.quit();
                true
            } else {
                false
            }
        }
        _ => false,
    });
    window.end();
    window
}
