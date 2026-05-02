use rusty_fractals::rusty::application;
use rusty_fractals::infra::config::MandelbrotVideo;
use rusty_fractals::rusty::fractal::FractalMath;
use rusty_fractals::calc::mem_collatz::MemCollatz;
use rusty_fractals::color::palettes::PaletteName::{BlueToWhiteCircleUp, LinearGray};

pub struct CollatzConjecture {}

impl FractalMath<MemCollatz> for CollatzConjecture {
    fn math(&self, m: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        m.square();
        m.collatz_conjecture();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = MandelbrotVideo {
        name: "Collatz Conjecture",

        iteration_max: 1348,
        palette: BlueToWhiteCircleUp,
        palette_zero: LinearGray,

        width_x: 1280,
        height_y: 720,
        width_re: 3.5,
        center_re: -0.088_485_445_553_580_48,
        center_im: -0.200_679_435_068_532_8,
    };

    application::execute(fractal_config.init(), CollatzConjecture {});
}

#[cfg(test)]
mod tests {
    use crate::CollatzConjecture;
    use rusty_fractals::rusty::fractal::{FractalMath, MemType};
    use rusty_fractals::calc::mem_collatz::MemCollatz;

    #[test]
    fn test_math() {
        let collatz = CollatzConjecture {};
        let mut mc = MemCollatz::new(0.0, 0.0);

        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.m.re, 2.0);
        assert_eq!(mc.m.im, 1.1);
    }
}
