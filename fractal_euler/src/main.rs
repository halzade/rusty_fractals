mod euler;
mod pixel;
mod mem_euler;

use rusty_fractals::fractal::{FractalMath};
use crate::mem_euler::MemEuler;

/*
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
*/
struct Euler {}

impl FractalMath<MemEuler> for Euler {
    fn math(&self, me: &mut MemEuler, origin_re: f64, origin_im: f64) {
        me.square();
        me.plus(origin_re, origin_im);
        me.euler();
        me.square();
        me.plus(origin_re, origin_im);
    }
}

fn main() {
    // TODO
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
