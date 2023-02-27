mod euler;
mod pixel;
mod mem_euler;
use rusty_fractals_common::area::Area;
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal::{Fractal, FractalMath};
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
struct Euler {
    name: &'static str,
}

impl FractalMath<MemEuler> for Euler {
    fn math(&self, me: &mut MemEuler, origin_re: f64, origin_im: f64) {
        me.square();
        me.plus(origin_re, origin_im);
        me.euler();
        me.square();
        me.plus(origin_re, origin_im);
    }
}

impl Fractal for Euler {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        todo!()
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, data_image: &DataImage, is_wrap: bool) -> (u32, u32) {
        todo!()
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

fn main() {
    // TODO
}