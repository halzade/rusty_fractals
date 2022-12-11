use color_palette::Palette;
use fractal_lib::mem::Mem;
use fractal_lib::resolution_multiplier;
use fractal_lib::color_palette;
use fractal_lib::color_palettes::PALETTE_BLACK_TO_WHITE;
use fractal_lib::fractal::{FractalConfig, FractalDefinition, Math};
use resolution_multiplier::ResolutionMultiplier;
use resolution_multiplier::ResolutionMultiplier::None;

const NAME: &str = "Fat Star Tentacle";

const ITERATION_MAX: u32 = 81_000;
const ITERATION_MIN: u32 = 8;
const AREA_SIZE: f64 = 3.5;
const TARGET_RE: f64 = 0.0;
const TARGET_IM: f64 = 0.0;
// TODO
// const INIT_FINEBROT_AREA_SIZE : f64= 0.5;
// const INIT_FINEBROT_TARGET_re : f64= 0.5;
// const INIT_FINEBROT_TARGET_im : f64= -0.38;
const RESOLUTION_WIDTH: u32 = 1920;
const RESOLUTION_HEIGHT: u32 = 1080;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = None;
const REPEAT: bool = false;
const SAVE_IMAGES: bool = false;
const PALETTE: Palette = PALETTE_BLACK_TO_WHITE;

struct FatStarTentacle {
    pub name: String,
}

impl Math for FatStarTentacle {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    println!("Started");

    let fat_start_tentacle = FatStarTentacle { name: NAME.to_string() };
    let definition = FractalDefinition { iteration_min: ITERATION_MIN, iteration_max: ITERATION_MAX, area_size: AREA_SIZE, target_re: TARGET_RE, target_im: TARGET_IM };
    let config = FractalConfig { resolution_width: RESOLUTION_WIDTH, resolution_height: RESOLUTION_HEIGHT, resolution_multiplier: RESOLUTION_MULTIPLIER, repeat: REPEAT, save_images: SAVE_IMAGES, palette: PALETTE };

    println!("Fractal {}", fat_start_tentacle.name);

    let mut m = Mem { re: 0.0, im: 0.0 };
    fat_start_tentacle.math(&mut m, 1.0, 0.1);

    println!("Finished.");
}


#[test]
fn test_math() {
    let fat_start_tentacle = FatStarTentacle { name: NAME.to_string() };
    let mut m = Mem { re: 0.0, im: 0.0 };
    fat_start_tentacle.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
