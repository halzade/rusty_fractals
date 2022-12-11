use color_palette::Palette;
use fractal_lib::mem::Mem;
use fractal_lib::resolution_multiplier;
use fractal_lib::color_palette;
use fractal_lib::color_palettes::PALETTE_BLACK_TO_WHITE;
use fractal_lib::fractal::{FractalConfig, FractalDefinition, Math};
use resolution_multiplier::ResolutionMultiplier;
use resolution_multiplier::ResolutionMultiplier::None;

const NAME: &str = "Lotus";
const ITERATION_MAX: u32 = 8000;
const ITERATION_MIN: u32 = 42;
const AREA_SIZE: f64 = 9.5;
const TARGET_RE: f64 = 0.67748277351478;
const TARGET_IM: f64 = -1.18770078111202;
const RESOLUTION_WIDTH: u32 = 1920;
const RESOLUTION_HEIGHT: u32 = 1080;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = None;
const REPEAT: bool = false;
const SAVE_IMAGES: bool = false;
const PALETTE: Palette = PALETTE_BLACK_TO_WHITE;

struct Lotus {
    pub name: String,
}

impl Math for Lotus {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    println!("Started");

    let lotus = Lotus { name: NAME.to_string() };
    let definition = FractalDefinition { iteration_min: ITERATION_MIN, iteration_max: ITERATION_MAX, area_size: AREA_SIZE, target_re: TARGET_RE, target_im: TARGET_IM };
    let config = FractalConfig { resolution_width: RESOLUTION_WIDTH, resolution_height: RESOLUTION_HEIGHT, resolution_multiplier: RESOLUTION_MULTIPLIER, repeat: REPEAT, save_images: SAVE_IMAGES, palette: PALETTE };

    println!("Fractal {}", lotus.name);

    let mut m = Mem { re: 0.0, im: 0.0 };
    lotus.math(&mut m, 1.0, 0.1);

    println!("Finished.");
}
