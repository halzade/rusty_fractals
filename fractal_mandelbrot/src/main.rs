use rusty_fractals::application;
use rusty_fractals::config::MandelbrotImage;
use rusty_fractals::fractal::FractalMath;
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, LinearGold};

/**
 * The Mandelbrot Fractal
 */
pub struct Mandelbrot {}

/**
 * x := x^2 + y^2 + x0
 * y := 2xy + y0
 */
impl FractalMath<Mem> for Mandelbrot {
    fn math(&self, mc: &mut Mem, origin_re: f64, origin_im: f64) {
        mc.square();
        mc.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = MandelbrotImage {
        name: "Mandelbrot",

        iteration_max: 2500,

        palette: BlueToWhiteCircleUp,
        palette_zero: LinearGold,

        width_x: 1280,
        height_y: 720,
        width_re: 4.5,
        center_re: -0.5,
        center_im: 0.0,
    };

    application::execute(fractal_config.init(), Mandelbrot {});
}

#[cfg(test)]
mod tests {
    use crate::Mandelbrot;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let mandelbrot = Mandelbrot {};
        let mut m = Mem::new(0.0, 0.0);

        mandelbrot.math(&mut m, 1.0, 1.0);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 1.0);
    }
}
