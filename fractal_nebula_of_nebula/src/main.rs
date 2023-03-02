use rusty_fractals_core::{machine};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{FractalConfig, Fractal, FractalMath, FractalName, Recalculate};
use rusty_fractals_common::fractal;
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square3;

struct NebulaOfNebula {}

impl FractalMath<Mem> for NebulaOfNebula {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        let r = m.re;
        let i = m.im;
        let or = origin_re;
        let oi = origin_im;
        // f(f(z)) : f = z^2 + c
        m.re = r * r * r * r - 6.0 * r * r * i * i + i * i * i * i + 2.0 * r * r * or - 2.0 * i * i * or - 4.0 * r * i * oi + or * or - oi * oi + or - r;
        m.im = 4.0 * r * r * r * i - 4.0 * r * i * i * i + 4.0 * r * i * or + 2.0 * r * r * oi - 2.0 * i * i * oi + 2.0 * or * oi + oi - i;
    }
}

impl Fractal for NebulaOfNebula {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::finite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, data: &DataImage, is_wrap: bool) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, data, is_wrap)
    }
}

impl FractalName for NebulaOfNebula {
    fn name(&self) -> &'static str { "Nebula of Nebula" }
}

impl Recalculate for NebulaOfNebula {
    fn recalculate() { todo!() }
}

fn main() {
    let fractal_config = FractalConfig {
        iteration_min: 42,
        iteration_max: 24800,
        resolution_multiplier: Square3,
        palette: palette_blue_to_white_circle_up(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 1000,
        width_re: 0.5,
        center_re: 0.0,
        center_im: 0.0,
    };
    let nebula_nebula = &NebulaOfNebula {};
    machine::nebula_calculation_for(nebula_nebula, fractal_config, area_config);
}
