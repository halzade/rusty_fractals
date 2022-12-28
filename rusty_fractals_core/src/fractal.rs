use fractal_stats::Stats;
use resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_domain::resolution_multiplier;
use crate::{fractal_stats, resolution_multiplier};
use crate::color_palette;
use crate::mem::Mem;
use crate::mem_collatz::MemCollatz;
use crate::mem_euler::MemEuler;
use crate::mem_phoenix::MemPhoenix;

pub const ITERATION_MAX: u32 = 6000;

/**
 * Image resolution height & width
 *  800  600
 * 1280  720
 * 1080 1920 full HD high
 * 1920 1080 full HD
 * 2560 1440 quad HD
 */

// Delete shorter paths then this
pub const TOLERATE_PATH_LENGTH_MIN: i32 = 4;
/**
 * 4 is quadrance from (0, 0)
 * If intermediate calculation result [re,im] spirals beyond this boundary. Calculation stops as divergent.
 */
pub const CALCULATION_BOUNDARY: i32 = 4;

pub struct Fractal {

}

pub struct FractalDefinition {
    // calculation config
    pub iteration_min: u32,
    pub iteration_max: u32,

    // image config
    pub resolution_multiplier: resolution_multiplier::ResolutionMultiplier,

    // app config
    pub repeat: bool,
    pub save_images: bool,

    // image config
    pub palette: color_palette::Palette,
}


pub trait Math {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64);
}

pub trait MathCollatz {
    fn math(&self, mc: &mut MemCollatz, origin_re: f64, origin_im: f64);
}

pub trait MathPhoenix {
    fn math(&self, mp: &mut MemPhoenix, origin_re: f64, origin_im: f64);
}

pub trait MathEuler {
    fn math(&self, me: &mut MemEuler, origin_re: f64, origin_im: f64);
}

pub trait Calculate {
    fn calculate(&self);
}

pub fn update(mut stats: Stats) {
    // TODO ITERATION_MAX += 150;

    stats.update(0); // TODO

    if stats.not_enough_pixels_best_value {
        // ("increase ITERATION_MAX, not enough Points");
        // TODO ITERATION_MAX += 20_000;
    }
    if stats.less_pixels_best_value {
        // TODO ITERATION_MAX += 2_000;
        // ("increase ITERATION_MAX, bit less Points");
    }
    if stats.too_many_paths_total {
        // ("increase a bit ITERATION_MIN, too many paths total");
        // TODO ITERATION_min += 1;
    }

    stats.print();
    stats.clean();
}
