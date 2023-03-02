use rusty_fractals_core::machine;
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{FractalConfig, Fractal, FractalMath, FractalName, Recalculate};
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;

struct FatStarTentacle {}

impl FractalMath<Mem> for FatStarTentacle {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for FatStarTentacle {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::infinite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, data: &DataImage, is_wrap: bool) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, data, is_wrap)
    }
}

impl FractalName for FatStarTentacle {
    fn name(&self) -> &'static str { "Fat Star" }
}

impl Recalculate for FatStarTentacle {
    fn recalculate() { todo!() }
}

fn main() {
    let fractal_config = FractalConfig {
        iteration_min: 8,
        iteration_max: 81000,
        resolution_multiplier: Single,
        palette: palette_blue_to_white_circle_up(),
    };
    // TODO
    // const INIT_FINEBROT_AREA_SIZE : f64= 0.5;
    // const INIT_FINEBROT_TARGET_re : f64= 0.5;
    // const INIT_FINEBROT_TARGET_im : f64= -0.38;
    let area_config = AreaConfig {
        width_x: 600,
        height_y: 600,
        width_re: 3.5,
        center_re: 0.0,
        center_im: 0.0,
    };
    let fat_star = &FatStarTentacle {};
    machine::nebula_calculation_for(fat_star, fractal_config, area_config);
}

#[test]
fn test_math() {
    let fat_star_tentacle = FatStarTentacle {};
    let mut m = Mem { re: 0.0, im: 0.0 };
    fat_star_tentacle.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
