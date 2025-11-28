use rusty_fractals::application;
use rusty_fractals::config::MandelbrotVideo;
use rusty_fractals::fractal::FractalMath;
use rusty_fractals::mem_collatz::MemCollatz;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, LinearBlue};

pub struct CollatzConjectureMandelbrot {}

impl FractalMath<MemCollatz> for CollatzConjectureMandelbrot {
    fn math(&self, mc: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        mc.square();
        mc.plus_collatz(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = MandelbrotVideo {
        name: "Collatz Conjecture Mandelbrot",

        iteration_max: 14800,
        palette: BlueToWhiteCircleUp,
        palette_zero: LinearBlue,

        width_x: 1280,
        height_y: 720,
        width_re: 3.0,
        center_re: -0.882952991714172300,
        center_im: -0.214699221335319460,
    };

    application::execute(fractal_config.init(), CollatzConjectureMandelbrot {});
}

#[cfg(test)]
mod tests {
    use crate::CollatzConjectureMandelbrot;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem_collatz::MemCollatz;

    #[test]
    fn test_math() {
        let collatz = CollatzConjectureMandelbrot {};
        let mut mc = MemCollatz::new(0.0, 0.0);

        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.m.re, 2.0);
        assert_eq!(mc.m.im, 0.65);
    }
}
