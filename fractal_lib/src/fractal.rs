use crate::fractal_stats;

pub const ITERATION_MAX: i32 = 6000;
pub const WIDTH_X: usize = 800;
pub const HEIGHT_Y: usize = 800;

// Delete shorter paths then this
pub const TOLERATE_PATH_LENGTH_MIN: i32 = 4;

pub const RESOLUTION_MULTIPLIER: i32 = 0; // TODO

pub const ITERATION_MIN: i32 = 0; // TODO

pub const INIT_FINEBROT_AREA_SIZE: f64 = 0.0;
// TODO
pub const INIT_FINEBROT_TARGET_RE: f64 = 0.0;
// TODO
pub const INIT_FINEBROT_TARGET_IM: f64 = 0.0; // TODO

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
