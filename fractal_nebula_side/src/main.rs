use rusty_fractals::application;
use rusty_fractals::fractal::FractalCalculationType::DynamicSequenceNebula;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, Nothing};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square2;

pub struct NebulaSide;

impl FractalMath<Mem> for NebulaSide {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = FractalConfig {
        name: "Nebula Side",
        fractal_calc_type: DynamicSequenceNebula,

        iteration_min: 42,
        iteration_max: 14800,
        resolution_multiplier: Square2,

        palette: BlueToWhiteCircleUp,
        palette_zero: Nothing,

        width_x: 1280,
        height_y: 720,
        width_re: 7.0,
        center_re: -0.10675625916322415,
        center_im: -0.8914368889277283,

        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };

    application::execute(fractal_config, NebulaSide {});
}

#[cfg(test)]
mod tests {
    use crate::NebulaSide;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let nebula = NebulaSide {};
        let mut m = Mem { re: 0.0, im: 0.0 };

        nebula.math(&mut m, 1.0, 0.1);

        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
