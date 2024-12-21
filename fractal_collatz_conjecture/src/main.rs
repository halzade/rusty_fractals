use rusty_fractals::application;
use rusty_fractals::fractal::CalculationType::InfiniteVideoZoom;
use rusty_fractals::fractal::FractalType::MandelbrotType;
use rusty_fractals::fractal::OrbitType::Ignore;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem_collatz::MemCollatz;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, GrayToBlackCircleDown};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;

pub struct CollatzConjecture {}

impl FractalMath<MemCollatz> for CollatzConjecture {
    fn math(&self, m: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        m.square();
        m.collatz_conjecture();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = FractalConfig {
        name: "Collatz Conjecture",
        iteration_min: 0,
        iteration_max: 1348,
        fractal_type: MandelbrotType,
        resolution_multiplier: Single,
        palette: BlueToWhiteCircleUp,
        palette_zero: GrayToBlackCircleDown,

        width_x: 1280,
        height_y: 720,
        width_re: 3.5,
        center_re: -0.088485445553580480,
        center_im: -0.200679435068532800,

        calc_type: InfiniteVideoZoom,
        orbits: Ignore,
        update_max: 0,
        update_min: 0,
    };

    application::execute(fractal_config, CollatzConjecture {});
}

#[cfg(test)]
mod tests {
    use crate::CollatzConjecture;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;
    use rusty_fractals::mem_collatz::MemCollatz;

    #[test]
    fn test_math() {
        let collatz = CollatzConjecture {};
        let mut mc = MemCollatz {
            m: Mem { re: 0.0, im: 0.0 },
            num: 7,
        };

        collatz.math(&mut mc, 1.0, 0.1);

        assert_eq!(mc.re(), 2.0);
        assert_eq!(mc.im(), 1.1);
    }
}
