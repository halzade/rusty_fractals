use ColorDepth::Rgb8;
use fltk::{frame::Frame, prelude::*, window::Window};
use fltk::app::App;
use fltk::enums::ColorDepth;
use fltk::image::RgbImage;

pub fn show(app_name: &'static str, domain_image: image::RgbImage, result_image: image::RgbImage) {
    let width = domain_image.width() as i32;
    let height = domain_image.height() as i32;

    let domain_image_rgb = RgbImage::new(&domain_image.into_raw(), width, height, Rgb8).unwrap();
    let result_image_rgb = RgbImage::new(&result_image.into_raw(), width, height, Rgb8).unwrap();

    let app = App::default();
    let mut wind = Window::new(100, 100, width * 2, height, app_name);
    let mut domain_frame = Frame::new(0, 0, width, height, "");
    let mut result_frame = Frame::new(width, 0, width, height, "");
    domain_frame.set_image(Some(domain_image_rgb));
    result_frame.set_image(Some(result_image_rgb));
    wind.add(&domain_frame);
    wind.add(&result_frame);
    wind.end();
    wind.show();
    app.run().unwrap();
}
