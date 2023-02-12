use rusty_fractals_common::fractal::Fractal;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_result::palette::Palette;
use rusty_fractals_core::mem_collatz::MemCollatz;

use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square3;
use rusty_fractals_result::palettes::palette_blue_to_white_circle_up;

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
// const PALETTE: Palette = palette_blue_to_white_circle_up();

struct CollatzConjecture {
    pub name: String,
}

impl Fractal<MemCollatz> for CollatzConjecture {
    fn math(&self, m: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        m.square();
        m.collatz_conjecture();
        m.plus(origin_re, origin_im);
    }
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        todo!()
    }
}

fn main() {
    // TODO
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