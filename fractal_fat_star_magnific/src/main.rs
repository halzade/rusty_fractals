use rusty_fractals_result::palette::Palette;
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::fractal::{FractalConfig, FractalDefinition, Math};
use rusty_fractals_domain::resolution_multiplier;
use resolution_multiplier::ResolutionMultiplier;
use resolution_multiplier::ResolutionMultiplier::None;
use rusty_fractals_result::palettes::palette_black_to_white;
use log::{info};

const NAME: &str = "Fat Star Magnific";
const ITERATION_MAX: u32 = 81_000;
const ITERATION_MIN: u32 = 8;
const AREA_SIZE: f64 = 3.5;
const TARGET_RE: f64 = 0.0;
const TARGET_IM: f64 = 0.0;
// TODO
// const INIT_FINEBROT_AREA_SIZE : f64= 0.15;
// const INIT_FINEBROT_TARGET_re : f64= 0.5425;
// const INIT_FINEBROT_TARGET_im : f64= -0.31;
const RESOLUTION_WIDTH: u32 = 1920;
const RESOLUTION_HEIGHT: u32 = 1080;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = None;
const REPEAT: bool = false;
const SAVE_IMAGES: bool = false;
const PALETTE: Palette = palette_black_to_white();

struct FatStarMagnific {
    pub name: String,
}

impl Math for FatStarMagnific {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    info!("Started");

    let fat_star_magnific = FatStarMagnific { name: NAME.to_string() };
    let definition = FractalDefinition { iteration_min: ITERATION_MIN, iteration_max: ITERATION_MAX, area_size: AREA_SIZE, target_re: TARGET_RE, target_im: TARGET_IM };
    let config = FractalConfig { resolution_width: RESOLUTION_WIDTH, resolution_height: RESOLUTION_HEIGHT, resolution_multiplier: RESOLUTION_MULTIPLIER, repeat: REPEAT, save_images: SAVE_IMAGES, palette: PALETTE };

    info!("Fractal {}", fat_star_magnific.name);

    let mut m = Mem { re: 0.0, im: 0.0 };
    fat_star_magnific.math(&mut m, 1.0, 0.1);

    info!("Finished.");
}


#[test]
fn test_math() {
    let fat_star_magnific = FatStarMagnific { name: NAME.to_string() };
    let mut m = Mem { re: 0.0, im: 0.0 };
    fat_star_magnific.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
