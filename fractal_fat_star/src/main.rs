use fractal_lib::mem::Mem;
use fractal_lib::resolution_multiplier;
use fractal_lib::color_palette;
use fractal_lib::color_palettes::PALETTE_BLACK_TO_WHITE;
use resolution_multiplier::ResolutionMultiplier::None;

struct FatStar {
    name: String,
    iteration_min: u32,
    iteration_max: u32,
    area_size: f64,
    target_re: f64,
    target_im: f64,
}

impl FatStar {
    pub fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

struct FractalConfig {
    resolution_width: u32,
    resolution_height: u32,
    resolution_multiplier: resolution_multiplier::ResolutionMultiplier,
    repeat: bool,
    save_images: bool,
    palette: color_palette::Palette,
}

fn main() {
    let name = String::from("Fat Star");
    println!("Fractal {}", name);

    /*
     * Fractal Definition
     */
    let fractal = FatStar {
        name,
        iteration_min: 42,
        iteration_max: 22000,
        area_size: 3.5,
        target_re: 0.0,
        target_im: 0.0
    };

    let config = FractalConfig {
        resolution_width: 1920,
        resolution_height: 1080,
        resolution_multiplier: None,
        repeat: false,
        save_images: false,
        palette: PALETTE_BLACK_TO_WHITE
    };

    let mut m = fractal_lib::mem::Mem { re: 0.0, im: 0.0 };
    fractal.math(&mut m, 1.0, 0.1);

    println!("Finished.");
}


#[test]
fn test_math() {
    let fractal = FatStar { name: String::from("FatStar"), iteration_min: 42, iteration_max: 22000, area_size: 3.5, target_re: 0.0, target_im: 0.0 };
    let mut m = fractal_lib::mem::Mem { re: 0.0, im: 0.0 };
    fractal.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}