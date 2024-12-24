use rusty_fractals::application;
use rusty_fractals::fractal::FractalCalculationType::StaticImageNebula;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, Nothing};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square5;

pub struct Science;

impl FractalMath<Mem> for Science {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = FractalConfig {
        name: "Science",
        fractal_calc_type: StaticImageNebula,
        orbits: Finite,

        iteration_min: 42,
        iteration_max: 1800,
        resolution_multiplier: Square5,

        palette: BlueToWhiteCircleUp,
        palette_zero: Nothing,

        width_x: 600,
        height_y: 600,
        width_re: 3.0,
        center_re: -0.5,
        center_im: 0.0,

        update_max: 150,
        update_min: 0,
    };

    application::execute(fractal_config, Science {});
}

#[cfg(test)]
mod tests {
    use crate::Science;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let fractal = Science;
        let mut m = Mem { re: 0.0, im: 0.0 };

        fractal.math(&mut m, 1.0, 0.1);

        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
