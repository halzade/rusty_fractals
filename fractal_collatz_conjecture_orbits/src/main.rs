use rusty_fractals::application;
use rusty_fractals::config::NebulaVideo;
use rusty_fractals::fractal::FractalMath;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::mem_collatz::MemCollatz;
use rusty_fractals::palettes::PaletteName::BlueToWhiteCircleUp;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square11;

pub struct CollatzConjectureOrbits {}

impl FractalMath<MemCollatz> for CollatzConjectureOrbits {
    fn math(&self, m: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        m.square();
        m.collatz_conjecture();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = NebulaVideo {
        name: "CollatzConjectureOrbits",

        iteration_min: 7,
        iteration_max: 1348,
        resolution_multiplier: Square11,
        palette: BlueToWhiteCircleUp,

        width_x: 1280,
        height_y: 720,
        width_re: 5.0,
        center_re: -0.088485445553580480,
        center_im: -0.200679435068532800,

        orbits: Infinite,
        update_max: 150,
        update_min: 0,
    };

    application::execute(fractal_config.init(), CollatzConjectureOrbits {});
}

#[cfg(test)]
mod tests {
    use crate::CollatzConjectureOrbits;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem_collatz::MemCollatz;

    #[test]
    fn test_math() {
        let collatz = CollatzConjectureOrbits {};
        let mut mc = MemCollatz::new(0.0, 0.0);

        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.m.re, 2.0);
        assert_eq!(mc.m.im, 1.1);
    }
}
