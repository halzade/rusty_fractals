use crate::data_image::DataImage;
use image::{ImageBuffer, RgbImage};

pub fn save_image(data_image: &DataImage, name: &str, index: u32) {
    println!("save_image()");

    let width = data_image.width_xp;
    let height = data_image.height_yp;

    if (width <= 800) || (height <= 800) {
        println!("save_image() {} {} skip", width, height);
        return;
    }

    let path = format!("{}_{}.jpg", to_snake(name), index);
    println!("{}", path);

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

fn to_snake(s: &str) -> String {
    s.to_lowercase().replace(' ', "_")
}

#[cfg(test)]
mod tests {
    use crate::files::{save_image, to_snake};
    use crate::fractal::init_trivial_dynamic_config;
    use crate::{area, data_image};

    #[test]
    fn test_save_image() {
        let fractal_name = "Fractal Snake";

        let c = init_trivial_dynamic_config(801);
        let a = area::init(&c);
        save_image(&data_image::init(&c, &a), fractal_name, 0);

        let file_name = "fractal_snake_0.jpg";
        assert!(std::fs::metadata(file_name).unwrap().is_file());

        std::fs::remove_file(file_name).unwrap();
    }

    #[test]
    fn test_to_snake() {
        let s = to_snake("Collatz Conjecture");
        assert_eq!(s, "collatz_conjecture");
    }
}
