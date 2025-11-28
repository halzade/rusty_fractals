use crate::data_image::DataImage;
use image::{ImageBuffer, RgbImage};

pub fn save_image(data_image: &DataImage) {
    println!("save_image()");

    let width = data_image.width_xp;
    let height = data_image.height_yp;
    let path = "fractal.jpg";

    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    // [0, height)
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

    use crate::files::save_image;
    use crate::fractal::init_trivial_static_config;
    use crate::{area, data_image};

    #[test]
    fn test_save_image() {
        let name = "fractal.jpg";

        let c = init_trivial_static_config();
        let a = area::init(&c);
        save_image(&data_image::init(&c, &a));

        // verify file created
        assert!(std::fs::metadata(name).unwrap().is_file());

        // cleanup
        std::fs::remove_file(name).unwrap();
    }
}
