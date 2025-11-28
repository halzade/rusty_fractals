use rusty_fractals::application;
use rusty_fractals::config::MandelbrotImage;
use rusty_fractals::fractal::FractalMath;
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, LinearRed};

pub struct MandelbrotOfMandelbrot {}

impl FractalMath<Mem> for MandelbrotOfMandelbrot {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        let r = m.re;
        let i = m.im;
        let or = origin_re;
        let oi = origin_im;
        // f(f(z)) : f = z^2 + c
        m.re = r * r * r * r - 6.0 * r * r * i * i + i * i * i * i + 2.0 * r * r * or
            - 2.0 * i * i * or
            - 4.0 * r * i * oi
            + or * or
            - oi * oi
            + or
            - r;
        m.im = 4.0 * r * r * r * i - 4.0 * r * i * i * i + 4.0 * r * i * or + 2.0 * r * r * oi
            - 2.0 * i * i * oi
            + 2.0 * or * oi
            + oi
            - i;
    }
}

fn main() {
    let fractal_config = MandelbrotImage {
        name: "Mandelbrot of Mandelbrot",
        iteration_max: 2500,

        palette: BlueToWhiteCircleUp,
        palette_zero: LinearRed,

        width_x: 1280,
        height_y: 720,
        width_re: 3.5,
        center_re: -0.5,
        center_im: 0.0,
    };

    application::execute(fractal_config.init(), MandelbrotOfMandelbrot {});
}

#[cfg(test)]
mod tests {
    use crate::MandelbrotOfMandelbrot;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let mandelbrot = MandelbrotOfMandelbrot {};
        let mut m = Mem::new(0.0, 0.0);

        mandelbrot.math(&mut m, 1.0, 1.0);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 3.0);
    }
}
