use std::path::Path;
use image::save_buffer_with_format;

pub fn same_image(image: image::RgbImage) {
    let width = image.width();
    let height = image.height();
    save_buffer_with_format(
        Path::new("fractal.jpg"),
        &*image.into_raw(),
        width,
        height,
        image::ColorType::Rgb8,
        image::ImageFormat::Jpeg,
    ).expect("could not safe image");
}