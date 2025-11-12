use crate::data_image::DataImage;
use image::{ImageBuffer, RgbImage};

pub fn save_image(data_image: &DataImage) {
    println!("save_image()");

    let width = data_image.width_x;
    let height = data_image.height_y;
    let path = "fractal.jpg";

    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            if let Some(color) = data_image.color_at(x, y) {
                img.put_pixel(x as u32, y as u32, color);
            }
        }
    }

    img.save(path).unwrap();
    println!("save_image() done");
}

#[cfg(test)]
mod tests {

    use crate::data_image;
    use crate::files::save_image;

    #[test]
    fn test_save_image() {
        let name = "fractal.jpg";
        save_image(&data_image::init_trivial());

        // verify file created
        assert!(std::fs::metadata(name).unwrap().is_file());

        // cleanup
        std::fs::remove_file(name).unwrap();
    }
}
