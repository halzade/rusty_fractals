use color_palette::Palette;
use fractal_lib::mem::Mem;
use fractal_lib::resolution_multiplier;
use fractal_lib::color_palette;
use fractal_lib::color_palettes::{PALETTE_BLACK_TO_WHITE, PALETTE_BLUE_TO_WHITE};
use fractal_lib::fractal::{FractalConfig, FractalDefinition, Math};
use fractal_lib::resolution_multiplier::ResolutionMultiplier::Square5;
use resolution_multiplier::ResolutionMultiplier;
use resolution_multiplier::ResolutionMultiplier::None;

const NAME: &str = "Glorious Head";
const ITERATION_MAX: u32 = 2500;
const ITERATION_MIN: u32 = 8;
const AREA_SIZE: f64 = 4.5;
const TARGET_RE: f64 = -0.16884290496519;
const TARGET_IM: f64 = -0.37573460559804;
const RESOLUTION_WIDTH: u32 = 1280;
const RESOLUTION_HEIGHT: u32 = 720;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = Square5;
const REPEAT: bool = true;
const SAVE_IMAGES: bool = false;
const PALETTE: Palette = PALETTE_BLUE_TO_WHITE;

struct GloriousHead {
    pub name: String,
}

impl Math for GloriousHead {
    fn math(&self, m: &mut MemPhonenix, origin_re: f64, origin_im: f64) {
        m.square();

        m.re += c;
        m.re += p * m.prev_prev_re;
        m.im += p * m.prev_prev_im;

        /* previous iteration */
        m.prev_prev_re = m.prev_re;
        m.prev_prev_im = m.prev_im;
        m.prev_re = m.re;
        m.prev_im = m.im;

        m.plus(originRe, originIm);
    }
}

fn main() {
    println!("Started");

    c = 0.35;
    p = -0.25;
    phoenix_initializer = 1;

    let glorious_head = GloriousHead { name: NAME.to_string() };
    let definition = FractalDefinition { iteration_min: ITERATION_MIN, iteration_max: ITERATION_MAX, area_size: AREA_SIZE, target_re: TARGET_RE, target_im: TARGET_IM };
    let config = FractalConfig { resolution_width: RESOLUTION_WIDTH, resolution_height: RESOLUTION_HEIGHT, resolution_multiplier: RESOLUTION_MULTIPLIER, repeat: REPEAT, save_images: SAVE_IMAGES, palette: PALETTE };

    println!("Fractal {}", glorious_head.name);

    let mut m = fractal_lib::mem::Mem { re: 0.0, im: 0.0 };
    glorious_head.math(&mut m, 1.0, 0.1);

    println!("Finished.");
}

#[test]
fn test_math() {
    let glorious_head = GloriousHead { name: NAME.to_string() };
    let mut m = fractal_lib::mem::Mem { re: 0.0, im: 0.0 };
    glorious_head.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
