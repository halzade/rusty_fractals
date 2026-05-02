use rusty_fractals::rusty::application;
use rusty_fractals::infra::config::MandelbrotVideo;
use rusty_fractals::rusty::fractal::FractalMath;
use rusty_fractals::calc::mem_collatz::MemCollatz;
use rusty_fractals::color::palettes::PaletteName::{BlueToWhiteCircleUp, LinearBlue};

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
        center_re: -0.882_952_991_714_172_3,
        center_im: -0.214_699_221_335_319_46,
    };

    application::execute(fractal_config.init(), CollatzConjectureMandelbrot {});
}

#[cfg(test)]
mod tests {
    use crate::CollatzConjectureMandelbrot;
    use rusty_fractals::rusty::fractal::{FractalMath, MemType};
    use rusty_fractals::calc::mem_collatz::MemCollatz;

    #[tokio::test]
    async fn test_math() {
        let collatz = CollatzConjectureMandelbrot {};
        let mut mc = MemCollatz::new(0.0, 0.0);

        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.m.re, 2.0);
        assert_eq!(mc.m.im, 0.65);
    }
}
