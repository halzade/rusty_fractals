use crate::fractal_stats;
use crate::resolution_multiplier;
use crate::color_palette;
use crate::mem::Mem;

pub const ITERATION_MAX: i32 = 6000;

/**
 * Image resolution height & width
 *  800  600
 * 1280  720
 * 1080 1920 full HD high
 * 1920 1080 full HD
 * 2560 1440 quad HD
 */
pub const WIDTH_X: usize = 800;
pub const HEIGHT_Y: usize = 800;

// Delete shorter paths then this
pub const TOLERATE_PATH_LENGTH_MIN: i32 = 4;
/**
 * 4 is quadrance from (0, 0)
 * If intermediate calculation result [re,im] spirals beyond this boundary. Calculation stops as divergent.
 */
pub const CALCULATION_BOUNDARY: i32 = 4;

pub const RESOLUTION_MULTIPLIER: i32 = 0; // TODO

pub const ITERATION_MIN: i32 = 0; // TODO

pub const INIT_FINEBROT_AREA_SIZE: f64 = 0.0;
// TODO
pub const INIT_FINEBROT_TARGET_RE: f64 = 0.0;
// TODO
pub const INIT_FINEBROT_TARGET_IM: f64 = 0.0; // TODO

pub struct FractalDefinition {
    pub iteration_min: u32,
    pub iteration_max: u32,
    pub area_size: f64,
    pub target_re: f64,
    pub target_im: f64,
}

pub struct FractalConfig {
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub resolution_multiplier: resolution_multiplier::ResolutionMultiplier,

    pub repeat: bool,
    pub save_images: bool,
    pub palette: color_palette::Palette,
}


pub trait Math {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64);
}

// pub fn update(mut stats: fractal_stats::Stats) {
//     // TODO ITERATION_MAX += 150;
//
//     stats.update(0); // TODO
//
//     if stats.not_enough_pixels_best_value {
//         // ("increase ITERATION_MAX, not enough Points");
//         // TODO ITERATION_MAX += 20_000;
//     }
//     if stats.less_pixels_best_value {
//         // TODO ITERATION_MAX += 2_000;
//         // ("increase ITERATION_MAX, bit less Points");
//     }
//     if stats.too_many_paths_total {
//         // ("increase a bit ITERATION_MIN, too many paths total");
//         // TODO ITERATION_min += 1;
//     }
//
//     stats.print();
//     stats.clean();
// }
