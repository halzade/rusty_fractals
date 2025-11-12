use rusty_fractals::application;
use rusty_fractals::fractal::FractalCalculationType::StaticImageNebula;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{BlackWBWB, Nothing};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;

pub struct InfinityPerfection {}

impl FractalMath<Mem> for InfinityPerfection {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = FractalConfig {
        name: "InfinityPerfection",
        fractal_calc_type: StaticImageNebula,

        iteration_min: 3000,
        iteration_max: 180_000,
        resolution_multiplier: Single,
        palette: BlackWBWB,

        palette_zero: Nothing,
        width_x: 400,
        height_y: 800,
        width_re: 0.5,
        center_re: -1.25,
        center_im: 0.0,

        orbits: Infinite,
        update_max: 150,
        update_min: 0,
    };

    application::execute(fractal_config, InfinityPerfection {});
}

#[cfg(test)]
mod tests {
    use crate::InfinityPerfection;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let nebula = InfinityPerfection {};
        let mut m = Mem { re: 0.0, im: 0.0 };
        nebula.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
