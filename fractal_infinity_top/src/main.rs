use color_palette::Palette;
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::fractal::{FractalConfig, FractalDefinition, Math};
use rusty_fractals_domain::resolution_multiplier;
use rusty_fractals_result::color_palette;
use rusty_fractals_result::color_palettes::PALETTE_BLACK_TO_WHITE;
use resolution_multiplier::ResolutionMultiplier;
use resolution_multiplier::ResolutionMultiplier::None;
use log::{info};

const NAME: &str = "Infinity Top";
const ITERATION_MAX: u32 = 180_000;
const ITERATION_MIN: u32 = 3000;
const AREA_SIZE: f64 = 2.5;
const TARGET_RE: f64 = -0.5;
const TARGET_IM: f64 = 0.0;
// INIT_FINEBROT_AREA_SIZE = 1.8;
// INIT_FINEBROT_TARGET_re = -1.0;
// INIT_FINEBROT_TARGET_im = 0.0;
const RESOLUTION_WIDTH: u32 = 1920;
const RESOLUTION_HEIGHT: u32 = 1080;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = None;
const REPEAT: bool = false;
const SAVE_IMAGES: bool = false;
const PALETTE: Palette = PALETTE_BLACK_TO_WHITE;

struct InfinityTop {
    pub name: String,
}

impl Math for InfinityTop {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    info!("Started");

    let infinity_top = InfinityTop { name: NAME.to_string() };
    let definition = FractalDefinition { iteration_min: ITERATION_MIN, iteration_max: ITERATION_MAX, area_size: AREA_SIZE, target_re: TARGET_RE, target_im: TARGET_IM };
    let config = FractalConfig { resolution_width: RESOLUTION_WIDTH, resolution_height: RESOLUTION_HEIGHT, resolution_multiplier: RESOLUTION_MULTIPLIER, repeat: REPEAT, save_images: SAVE_IMAGES, palette: PALETTE };

    info!("Fractal {}", infinity_top.name);

    let mut m = Mem { re: 0.0, im: 0.0 };
    infinity_top.math(&mut m, 1.0, 0.1);

    info!("Finished.");
}


#[test]
fn test_math() {
    let infinity_top = InfinityTop { name: NAME.to_string() };
    let mut m = Mem { re: 0.0, im: 0.0 };
    infinity_top.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
