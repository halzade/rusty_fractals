use crate::data_image::DataImage;
use image::{ImageBuffer, RgbImage};

pub fn save_image(data_image: &DataImage, name: &str, index: u64) {
    println!("save_image()");

    let width = data_image.width_xl;
    let height = data_image.height_yl;

    if width <= 600 {
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

    if let Err(e) = img.save(&path) {
        eprintln!("Failed to save image {}: {:?}", path, e);
    }
    println!("save_image() done");
}

fn to_snake(s: &str) -> String {
    s.to_lowercase().replace(' ', "_")
}

#[cfg(test)]
mod tests {
    use crate::infra::files::{save_image, to_snake};
    use crate::rusty::fractal::init_trivial_dynamic_config;
    use crate::data_image;
    use crate::domain::area;

    #[test]
    fn test_save_image() {
        let fractal_name = "Fractal Snake";
        let c = init_trivial_dynamic_config(621);
        let a = area::init(&c);

        save_image(&data_image::init(&c, &a), fractal_name, 0);

        let file_name = "fractal_snake_0.jpg";
        if let Ok(meta) = std::fs::metadata(file_name) {
            assert!(meta.is_file());
        }

        if let Ok(img) = image::open(file_name) {
            assert_eq!(img.width(), 620);
            assert_eq!(img.height(), 620);
        }

        let _ = std::fs::remove_file(file_name);
    }

    #[test]
    fn test_to_snake() {
        let s = to_snake("Collatz Conjecture");
        assert_eq!(s, "collatz_conjecture");
    }
}
