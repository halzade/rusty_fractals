use rusty_fractals::area::AreaConfig;
use rusty_fractals::calc::CalculationConfig;
use rusty_fractals::calc::OrbitType::Infinite;
use rusty_fractals::fractal::{FractalMath, MandelbrotConfig};
use rusty_fractals::mem_collatz::MemCollatz;
use rusty_fractals::palettes::{palette_blue_to_white_circle_up, palette_gray_to_blue};
use rusty_fractals::application::Application;
use rusty_fractals::{application, window};
use std::thread;

pub struct CollatzConjectureMandelbrot<'lt> {
    app: Application<'lt>,
}

impl FractalMath<MemCollatz> for CollatzConjectureMandelbrot<'_> {
    fn math(&self, mc: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        mc.square();
        mc.plus_collatz(origin_re, origin_im);
    }
}

fn main() {
    let name: &'static str = "Collatz Conjecture Mandelbrot";
    let mandelbrot_config: MandelbrotConfig<'static> = MandelbrotConfig {
        iteration_max: 14800,
        palette: palette_blue_to_white_circle_up(),
        palette_zero: palette_gray_to_blue(),
        phantom: Default::default(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 720,
        width_re: 3.0,
        center_re: -0.882952991714172300,
        center_im: -0.214699221335319460,
    };
    let calculation_config = CalculationConfig {
        orbits: Infinite,
        update_max: 150,
        update_min: 0,
    };
    let application: Application<'static> = application::init(area_config, mandelbrot_config);
    let mut mandelbrot: CollatzConjectureMandelbrot<'static> =
        CollatzConjectureMandelbrot { app: application };
    let app = window::show(&mandelbrot);
    thread::spawn(move || {
        // TODO mandelbrot.calculate_mandelbrot();
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::CollatzConjectureMandelbrot;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;
    use rusty_fractals::mem_collatz::MemCollatz;
    use rusty_fractals::application;

    #[test]
    fn test_math() {
        let collatz = CollatzConjectureMandelbrot {
            app: application::init_none(),
        };
        let mut mc = MemCollatz {
            m: Mem { re: 0.0, im: 0.0 },
            num: 0,
        };
        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.re(), 2.0);
        assert_eq!(mc.im(), 0.65);
    }
}
