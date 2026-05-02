/*
 * cargo clippy
 * cargo clippy --release
 */
#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::panic)]
#![forbid(clippy::todo)]
#![forbid(clippy::unimplemented)]
#![deny(warnings)]
#![deny(clippy::all)]
/*
 * Implementation
 */
extern crate core;
pub mod calc {
    pub mod mathematician;
    pub mod mem;
    pub mod mem_collatz;
    pub mod mem_phoenix;
}
pub mod color {
    pub mod palette;
    pub mod palette_utils;
    pub mod palettes;
    pub mod perfect_color_distribution;
    pub mod perfect_color_distribution_euler;
    pub mod perfect_color_distribution_nebula;
}
pub mod domain {
    pub mod area;
    pub mod data_px;
    pub mod data_px3;
    pub mod resolution_multiplier;
}
pub mod image {
    pub mod data_image;
    pub mod pixel;
    pub mod pixel_states;
}
pub mod infra {
    pub mod config;
    pub mod constants;
    pub mod error;
    pub mod euler;
    pub mod files;
    pub mod fractal_log;
    pub mod fractal_stats;
}
pub mod rusty {
    pub mod application;
    pub mod fractal;
    pub mod machine;
}
