use std::path::Path;
use image::ColorType::Rgb8;
use image::ImageFormat::Jpeg;
use image::save_buffer_with_format;

pub fn save_image(image: image::RgbImage) {
    let width = image.width();
    let height = image.height();
    let path = "fractal.jpg";
    save_buffer_with_format(Path::new(path), &*image.into_raw(), width, height, Rgb8, Jpeg).expect("could not safe image");
}