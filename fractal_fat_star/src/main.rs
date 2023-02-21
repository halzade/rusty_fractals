use rusty_fractals_core::{machine, window};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{CalculationConfig, Fractal, FractalMath};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals_common::result_data_static::ResultDataStatic;
use rusty_fractals_result::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_result::result::ResultConfig;

struct FatStar {}

impl FractalMath<Mem> for FatStar {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for FatStar {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::infinite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, result_static: &ResultDataStatic) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, result_static)
    }
}


fn main() {
    let name = "Fat Star";

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;

    let calculation_config = CalculationConfig {
        iteration_min: 42,
        iteration_max: 22000,
        resolution_multiplier: Single,
    };
    let area_config = AreaConfig {
        width_re: 3.5,
        center_re: 0.0,
        center_im: 0.0,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_blue_to_white_circle_up(),
    };

    let fat_star = FatStar {};
    let machine = machine::init(&calculation_config, &result_config, &area_config);
    let (domain_image, result_image) = machine.calculate(&fat_star);

    window::show(name, domain_image, &result_image);
}

#[cfg(test)]
mod tests {
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use crate::FatStar;

    #[test]
    fn test_math() {
        let fat_star = FatStar {};
        let mut m = Mem { re: 0.0, im: 0.0 };
        fat_star.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}