use rusty_fractals_core::machine_mandelbrot;
use rusty_fractals_common::area::AreaConfig;
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{Conf, FractalMandelbrot, FractalMath, MandelbrotConfig, UpdateMandelbrot};
use rusty_fractals_common::mem_collatz::MemCollatz;
use rusty_fractals_common::palettes::{palette_blue_to_white_circle_up, palette_gray_to_blue};

struct CollatzConjectureMandelbrot {
    name: &'static str,
}

impl FractalMath<MemCollatz> for CollatzConjectureMandelbrot {
    fn math(&self, mc: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        mc.square();
        mc.plus_collatz(origin_re, origin_im);
    }
}

impl FractalMandelbrot for CollatzConjectureMandelbrot {
    fn calculate_mandelbrot_path(&self, iteration_max: u32, origin_re: f64, origin_im: f64) -> (u32, f64) {
        fractal::calculate_mandelbrot_path(self, iteration_max, origin_re, origin_im)
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

impl UpdateMandelbrot for CollatzConjectureMandelbrot {
    fn update(&self, conf: &mut Conf) {
        conf.max += 150;
        println!("iteration_max = {}", conf.max);
    }
}

fn main() {
    const WIDTH: usize = 1280;
    const HEIGHT: usize = 720;
    let mandelbrot_config = MandelbrotConfig {
        iteration_max: 14800,
        palette: palette_blue_to_white_circle_up(),
        palette_zero: palette_gray_to_blue(),
    };
    let area_config = AreaConfig {
        width_re: 3.0,
        center_re: -0.882952991714172300,
        center_im: -0.214699221335319460,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let collatz = &CollatzConjectureMandelbrot { name: "Collatz Conjecture Mandelbrot" };
    machine_mandelbrot::mandelbrot_calculation_for(collatz, WIDTH, HEIGHT, mandelbrot_config, area_config);
    // engine::calculate_mandelbrot_zoom(collatz, collatz, mandelbrot_config, area_config);
}

#[cfg(test)]
mod tests {
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_common::mem_collatz::MemCollatz;
    use crate::CollatzConjectureMandelbrot;

    #[test]
    fn test_math() {
        let collatz = CollatzConjectureMandelbrot { name: "Collatz" };
        let mut mc = MemCollatz { m: Mem { re: 0.0, im: 0.0 }, num: 0 };
        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.re(), 2.0);
        assert_eq!(mc.im(), 0.65);
    }
}
