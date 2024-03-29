use fltk::{frame::Frame, prelude::*, window::Window};
use fltk::app::{App, event_key};
use fltk::enums::{Event, Key};
use fltk::enums::ColorDepth::Rgb8;
use fltk::image::RgbImage;
use image::{ImageBuffer, Rgb};
use rusty_fractals_common::data_image::colour_for_state;
use rusty_fractals_common::pixel_states::DomainElementState;
use rusty_fractals_common::pixel_states::DomainElementState::{ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort, HibernatedDeepBlack};

const INT: i32 = 100;

pub fn show_state_colours() {
    let width = 600;
    let height = 100;
    let mut image = image::RgbImage::new(width as u32, height as u32);
    for y in 0..height {
        color_interval(&mut image, 0, 1, y, FinishedSuccessPast);
        color_interval(&mut image, 1, 2, y, FinishedSuccess);
        color_interval(&mut image, 2, 3, y, ActiveNew);
        color_interval(&mut image, 3, 4, y, FinishedTooShort);
        color_interval(&mut image, 4, 5, y, FinishedTooLong);
        color_interval(&mut image, 5, 6, y, HibernatedDeepBlack);
    }
    pop_app_window(width, height, image);
}

fn pop_app_window(width: i32, height: i32, image: ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let app = App::default();
    let image_rgb = RgbImage::new(image.as_raw(), width as i32, height as i32, Rgb8).unwrap();
    let mut window = Window::default().with_label("test window").with_size(width as i32, height as i32).center_screen();
    let mut frame = Frame::new(0, 0, width as i32, height as i32, "");
    frame.set_image(Some(image_rgb));
    window.add(&frame);
    window.handle(move |_, event| match event {
        Event::KeyDown => {
            let ek = event_key();
            if ek == Key::Escape {
                app.quit();
                return true;
            }
            false
        }
        _ => false,
    });
    window.end();
    window.show();
    app.run().unwrap();
}

fn color_interval(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, from: i32, to: i32, y: i32, state: DomainElementState) {
    for x in (from * INT)..(to * INT) {
        image.put_pixel(x as u32, y as u32, colour_for_state(state));
    }
}
