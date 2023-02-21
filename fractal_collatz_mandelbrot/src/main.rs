use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{CalculationConfig, Fractal, FractalMath};
use rusty_fractals_common::mem_collatz::MemCollatz;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals_common::result_data_static::ResultDataStatic;
use rusty_fractals_core::{machine, window};
use rusty_fractals_core::machine::Machine;
use rusty_fractals_result::palettes::palette_gray_to_blue;
use rusty_fractals_result::result::ResultConfig;

struct CollatzConjectureMandelbrot {}

impl FractalMath<MemCollatz> for CollatzConjectureMandelbrot {
    fn math(&self, mc: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        mc.square();
        mc.plus_collatz(origin_re, origin_im);
    }
}

impl Fractal for CollatzConjectureMandelbrot {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::finite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, result_static: &ResultDataStatic) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, result_static)
    }
}

fn main() {
    let name = "Collatz Conjecture Mandelbrot";

    const WIDTH: usize = 600; // 1280
    const HEIGHT: usize = 600; // 720

    let calculation_config = CalculationConfig {
        iteration_min: 0,
        iteration_max: 14800,
        resolution_multiplier: Single,
    };
    let area_config = AreaConfig {
        width_re: 3.0,
        center_re: -0.882952991714172300,
        center_im: -0.214699221335319460,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_gray_to_blue(),
    };

    let collatz = CollatzConjectureMandelbrot {};
    let machine: Machine = machine::init(&calculation_config, &result_config, &area_config);
    // TODO Mandelbrot type calculation
    let (domain_image, result_image) = machine.calculate(&collatz);

    window::show(name, domain_image, &result_image);
}

#[cfg(test)]
mod tests {
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_common::mem_collatz::MemCollatz;
    use crate::CollatzConjectureMandelbrot;

    #[test]
    fn test_math() {
        let collatz = CollatzConjectureMandelbrot {};
        let mut mc = MemCollatz { m: Mem { re: 0.0, im: 0.0 }, it: 0 };
        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.re(), 2.0);
        assert_eq!(mc.im(), 0.65);
    }
}
