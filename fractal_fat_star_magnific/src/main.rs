use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{CalculationConfig, Fractal, FractalMath};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::palettes::{palette_blue_to_white_circle_up, ResultConfig};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals_core::machine;

struct FatStarMagnific {
    name: &'static str,
}

impl FractalMath<Mem> for FatStarMagnific {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for FatStarMagnific {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::infinite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, data: &DataImage) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, data)
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

fn main() {
    const WIDTH: usize = 800; // 1920
    const HEIGHT: usize = 800; // 1080

    let calculation_config = CalculationConfig {
        iteration_min: 8,
        iteration_max: 81000,
        resolution_multiplier: Single,
    };
    // TODO
    // const INIT_FINEBROT_AREA_SIZE : f64= 0.15;
    // const INIT_FINEBROT_TARGET_re : f64= 0.5425;
    // const INIT_FINEBROT_TARGET_im : f64= -0.31;

    let area_config = AreaConfig {
        width_re: 3.0,
        center_re: 0.0,
        center_im: 0.0,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_blue_to_white_circle_up(),
    };

    let fat_star = &FatStarMagnific { name: "Fat Star Magnific" };
    machine::nebula_calculation_for(fat_star, WIDTH, HEIGHT, calculation_config, result_config, area_config);
}

#[test]
fn test_math() {
    let fat_star_magnific = FatStarMagnific { name: "Fat Star" };
    let mut m = Mem { re: 0.0, im: 0.0 };
    fat_star_magnific.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
