use color_palette::Palette;
use rusty_fractals_core::fractal::{FractalConfig, FractalDefinition, MathCollatz};
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::mem_collatz::MemCollatz;
use rusty_fractals_domain::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_domain::resolution_multiplier::ResolutionMultiplier::None;
use rusty_fractals_image::color_palette;
use rusty_fractals_image::color_palettes::{PALETTE_BLUE_TO_WHITE, PALETTE_GRAY_TO_BLUE};

const NAME: &str = "Collatz Conjecture Mandelbrot";
const ITERATION_MAX: u32 = 14800;
const AREA_SIZE: f64 = 3.0;
const TARGET_RE: f64 = -0.882952991714172300;
const TARGET_IM: f64 = -0.214699221335319460;
const RESOLUTION_WIDTH: u32 = 1280;
const RESOLUTION_HEIGHT: u32 = 720;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = None;
const REPEAT: bool = true;
const SAVE_IMAGES: bool = false;
const PALETTE: Palette = PALETTE_BLUE_TO_WHITE;
// to paint the black insides of the Mandelbrot set
const PALETTE_ZERO: Palette = PALETTE_GRAY_TO_BLUE;

struct CollatzConjectureMandelbrot {
    pub name: String,
}

impl MathCollatz for CollatzConjectureMandelbrot {
    fn math(&self, mc: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        mc.m.square();
        mc.plus_collatz(origin_re, origin_im);
    }
}

fn main() {
    println!("Started");

    let collatz = CollatzConjectureMandelbrot { name: NAME.to_string() };
    let definition = FractalDefinition { iteration_min: 0, iteration_max: ITERATION_MAX, area_size: AREA_SIZE, target_re: TARGET_RE, target_im: TARGET_IM };
    let config = FractalConfig { resolution_width: RESOLUTION_WIDTH, resolution_height: RESOLUTION_HEIGHT, resolution_multiplier: RESOLUTION_MULTIPLIER, repeat: REPEAT, save_images: SAVE_IMAGES, palette: PALETTE };

    println!("Fractal {}", collatz.name);

    let m = Mem { re: 0.0, im: 0.0 };
    let mut mc = MemCollatz { m, it: 0 };
    collatz.math(&mut mc, 1.0, 0.1);

    println!("Finished.");
}


#[test]
fn test_math() {
    let collatz = CollatzConjectureMandelbrot { name: NAME.to_string() };
    let m = Mem { re: 0.0, im: 0.0 };
    let mut mc = MemCollatz { m, it: 0 };
    collatz.math(&mut mc, 1.0, 0.1);
    assert_eq!(mc.re(), 2.0);
    assert_eq!(mc.im(), 0.65);
}
