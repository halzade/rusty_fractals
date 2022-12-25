use color_palette::Palette;
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::fractal::{FractalConfig, FractalDefinition, Math};
use rusty_fractals_domain::resolution_multiplier;
use rusty_fractals_image::palette;
use rusty_fractals_image::palettes::PALETTE_BLUE_TO_WHITE;
use resolution_multiplier::ResolutionMultiplier;
use resolution_multiplier::ResolutionMultiplier::SquareAlter;

const NAME: &str = "Nebula";

struct Nebula {
    pub name: String,
}

impl Math for Nebula {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    println!("Started");

    let nebula = Nebula { name: NAME.to_string() };
    let definition = FractalDefinition {
        iteration_min: 42,
        iteration_max: 14800,
        area_size: 7.0,
        target_re: 0.0,
        target_im: 0.0,
        resolution_width: 1280,
        resolution_height: 720,
        resolution_multiplier: SquareAlter,
        repeat: false,
        save_images: false,
        palette: PALETTE_BLUE_TO_WHITE
    };

    println!("Fractal {}", nebula.name);

    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);

    println!("Finished.");
}


#[test]
fn test_math() {
    let nebula = Nebula { name: NAME.to_string() };
    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
