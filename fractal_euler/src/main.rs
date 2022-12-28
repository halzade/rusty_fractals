mod euler;
mod pixel;
mod mandelbrot;

use color_palette::Palette;
use rusty_fractals_core::fractal::{FractalConfig, FractalDefinition, MathEuler};
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::mem_euler::{MemEuler, Spectra};
use rysty_fractals_core::mathematician::Mathematician;
use rusty_fractals_domain::resolution_multiplier::ResolutionMultiplier::None;
use rusty_fractals_domain::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_result::color_palette;
use rusty_fractals_result::color_palettes::{PALETTE_3_RGB};
use rusty_fractals_core::mathematician::Mathematician;

const NAME: &str = "Euler";

const ITERATION_MAX: u32 = 80000;
const ITERATION_MIN: u32 = 42;
const AREA_SIZE: f64 = 4.0;
const TARGET_RE: f64 = 0.0;
const TARGET_IM: f64 = 0.0;
const RESOLUTION_WIDTH: u32 = 1920;
const RESOLUTION_HEIGHT: u32 = 1080;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = None;
const REPEAT: bool = false;
const SAVE_IMAGES: bool = false;
const PALETTE: Palette = PALETTE_3_RGB;

struct Euler {
    pub name: String,
}

impl MathEuler for Euler {
    fn math(&self, mp: &mut MemEuler, origin_re: f64, origin_im: f64) {
        mp.m.square();
        mp.m.plus(origin_re, origin_im);
        mp.euler();
        mp.m.square();
        mp.m.plus(origin_re, origin_im);
    }
}

fn main() {
    println!("Started");

    let euler = Euler { name: NAME.to_string() };
    let definition = FractalDefinition { iteration_min: ITERATION_MIN, iteration_max: ITERATION_MAX, area_size: AREA_SIZE, target_re: TARGET_RE, target_im: TARGET_IM };
    let config = FractalConfig { resolution_width: RESOLUTION_WIDTH, resolution_height: RESOLUTION_HEIGHT, resolution_multiplier: RESOLUTION_MULTIPLIER, repeat: REPEAT, save_images: SAVE_IMAGES, palette: PALETTE };

    println!("Fractal {}", euler.name);

    let m = Mem { re: 0.0, im: 0.0 };
    let math = Mathematician {
        primes: Mathematician::init_primes(),
        fibonacci: Default::default(),
        perfect: Default::default(),
        square: Default::default(),
    };
    let mut me = MemEuler { m, it: 0, math, spectra: Spectra::Red }; // TODO

    euler.math(&mut me, 1.0, 0.1);

    println!("Finished.");
}

#[test]
fn test_math() {
    let euler = Euler { name: NAME.to_string() };
    let m = Mem { re: 0.0, im: 0.0 };
    let math = Mathematician {
        primes: Mathematician::init_primes(),
        fibonacci: Default::default(),
        perfect: Default::default(),
        square: Default::default()
    };
    let mut me = MemEuler { m, it: 0, math, spectra: Spectra::Red }; // TODO

    euler.math(&mut me, 1.0, 0.1);

    assert_eq!(me.re(), 0.9901);
    assert_eq!(me.im(), 0.10200000000000001);
}