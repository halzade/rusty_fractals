mod euler;
mod pixel;
mod mandelbrot;
mod mem_euler;

use rusty_fractals_common::fractal::Fractal;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_core::mathematician::Mathematician;
use crate::mem_euler::MemEuler;
use crate::pixel::Spectra;

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
struct Euler {
    pub name: String,
}

impl Fractal<MemEuler> for Euler {
    fn math(&self, mp: &mut MemEuler, origin_re: f64, origin_im: f64) {
        mp.m.square();
        mp.m.plus(origin_re, origin_im);
        mp.euler();
        mp.m.square();
        mp.m.plus(origin_re, origin_im);
    }

    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        todo!()
    }
}

fn main() {
    // TODO
}