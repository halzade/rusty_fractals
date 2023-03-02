use rusty_fractals_core::machine;
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{FractalConfig, Fractal, FractalMath, FractalName, Recalculate};
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;

struct InfinityTop {}

impl FractalMath<Mem> for InfinityTop {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for InfinityTop {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::infinite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, data: &DataImage, is_wrap: bool) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, data, is_wrap)
    }
}

impl FractalName for InfinityTop {
    fn name(&self) -> &'static str { "Infinity" }
}

impl Recalculate for InfinityTop {
    fn recalculate() { todo!() }
}

fn main() {
    let fractal_config = FractalConfig {
        iteration_min: 3000,
        iteration_max: 30_000, // 180_000
        resolution_multiplier: Single,
        palette: palette_blue_to_white_circle_up(),
    };
    // TODO
    // INIT_FINEBROT_AREA_SIZE = 1.8;
    // INIT_FINEBROT_TARGET_re = -1.0;
    // INIT_FINEBROT_TARGET_im = 0.0;
    let area_config = AreaConfig {
        width_x: 600,
        height_y: 600,
        width_re: 2.5,
        center_re: -0.5,
        center_im: 0.0,
    };
    let infinity = &InfinityTop {};
    machine::nebula_calculation_for(infinity, fractal_config, area_config);
}

#[test]
fn test_math() {
    let infinity_top = InfinityTop {};
    let mut m = Mem { re: 0.0, im: 0.0 };
    infinity_top.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
