use rusty_fractals::area::AreaConfig;
use rusty_fractals::fractal::{FractalMath, MandelbrotConfig};
use rusty_fractals::mem_collatz::MemCollatz;
use rusty_fractals::palettes::{
    palette_blue_to_white_circle_up, palette_gray_to_black_circle_down,
};
use rusty_fractals::application::Application;
use rusty_fractals::{application, window};
use std::thread;

pub struct CollatzConjecture<'lt> {
    app: Application<'lt>,
}

impl FractalMath<MemCollatz> for CollatzConjecture<'_> {
    fn math(&self, m: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        m.square();
        m.collatz_conjecture();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let name: &'static str = "Collatz Conjecture";
    let mandelbrot_config: MandelbrotConfig<'static> = MandelbrotConfig {
        iteration_max: 1348,
        palette: palette_blue_to_white_circle_up(),
        palette_zero: palette_gray_to_black_circle_down(),
        phantom: Default::default(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 720,
        width_re: 3.5,
        center_re: -0.088485445553580480,
        center_im: -0.200679435068532800,
    };
    let application: Application<'static> = application::init(area_config, mandelbrot_config);
    let mut mandelbrot: CollatzConjecture<'static> = CollatzConjecture { app: application };
    let app = window::show(&mandelbrot);
    thread::spawn(move || {
        // TODO mandelbrot.calculate_mandelbrot();
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::CollatzConjecture;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;
    use rusty_fractals::mem_collatz::MemCollatz;
    use rusty_fractals::application;

    #[test]
    fn test_math() {
        let collatz = CollatzConjecture {
            app: application::init_none(),
        };
        let mut mc = MemCollatz {
            m: Mem { re: 0.0, im: 0.0 },
            num: 7,
        };
        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.re(), 2.0);
        assert_eq!(mc.im(), 1.1);
    }
}
