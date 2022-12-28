use color_palette::Palette;
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::fractal::{FractalConfig, FractalDefinition, Math};
use rusty_fractals_domain::{domain, domain_area, resolution_multiplier};
use rusty_fractals_result::color_palette;
use rusty_fractals_result::color_palettes::PALETTE_BLACK_TO_WHITE;
use resolution_multiplier::ResolutionMultiplier;
use resolution_multiplier::ResolutionMultiplier::None;

const NAME: &str = "Fat Star";

struct FatStar {
    pub name: String,
}

impl Math for FatStar {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    println!("Started");

    let fat_star = FatStar { name: NAME.to_string() };
    let definition = FractalDefinition {
        iteration_min: 42,
        iteration_max: 22000,
        area_size: 3.5,
        target_re: 0.0,
        target_im: 0.0,
        resolution_width: 1920,
        resolution_height: 1080,
        resolution_multiplier: None,
        repeat: false,
        save_images: false,
        palette: PALETTE_BLACK_TO_WHITE
    };
    let area = domain_area::init(AREA_SIZE, TARGET_RE, TARGET_IM);
    let domain = domain_area::init_domain_elements(area);
    let engine = fractal_engine::Engine{domain, fat_star};
    println!("Fractal {}", fat_star.name);

    let mut m = Mem { re: 0.0, im: 0.0 };
    fat_star.math(&mut m, 1.0, 0.1);

    println!("Finished.");
}


#[test]
fn test_math() {
    let fat_star = FatStar { name: NAME.to_string() };
    let mut m = Mem { re: 0.0, im: 0.0 };
    fat_star.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}