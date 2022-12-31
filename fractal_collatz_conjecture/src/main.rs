use color_palette::Palette;
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::fractal::{FractalConfig, FractalDefinition, Math, MathCollatz};
use rusty_fractals_core::mem_collatz::MemCollatz;
use rusty_fractals_domain::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_domain::resolution_multiplier::ResolutionMultiplier::None;
use rusty_fractals_domain::resolution_multiplier::ResolutionMultiplier::Square3;
use rusty_fractals_result::color_palettes::{PALETTE_BLACK_TO_WHITE, PALETTE_BLUE_TO_WHITE};
use rusty_fractals_result::color_palette;
use log::{info};

const NAME: &str = "Collatz Conjecture";
const ITERATION_MAX: u32 = 14800;
const ITERATION_MIN: u32 = 42;
const AREA_SIZE: f64 = 7.0;
const TARGET_RE: f64 = -0.088485445553580480;
const TARGET_IM: f64 = -0.200679435068532800;
const RESOLUTION_WIDTH: u32 = 1280;
const RESOLUTION_HEIGHT: u32 = 720;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = Square3;
const REPEAT: bool = true;
const SAVE_IMAGES: bool = false;
const PALETTE: Palette = PALETTE_BLUE_TO_WHITE;

struct CollatzConjecture {
    pub name: String,
}

impl MathCollatz for CollatzConjecture {
    fn math(&self, m: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        m.square();
        m.collatz_conjecture();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    info!("Started");

    let collatz = CollatzConjecture { name: NAME.to_string() };
    let definition = FractalDefinition { iteration_min: ITERATION_MIN, iteration_max: ITERATION_MAX, area_size: AREA_SIZE, target_re: TARGET_RE, target_im: TARGET_IM };
    let config = FractalConfig { resolution_width: RESOLUTION_WIDTH, resolution_height: RESOLUTION_HEIGHT, resolution_multiplier: RESOLUTION_MULTIPLIER, repeat: REPEAT, save_images: SAVE_IMAGES, palette: PALETTE };

    info!("Fractal {}", collatz.name);

    let m = Mem { re: 0.0, im: 0.0 };
    let mut mc = MemCollatz { m, it: 1 };
    collatz.math(&mut mc, 1.0, 0.1);

    info!("Finished.");
}

#[test]
fn test_math() {
    let collatz = CollatzConjecture { name: NAME.to_string() };
    let m = Mem { re: 0.0, im: 0.0 };
    let mut mc = MemCollatz { m, it: 1 };
    collatz.math(&mut mc, 1.0, 0.1);
    assert_eq!(mc.re(), 2.0);
    assert_eq!(mc.im(), 1.1);
}