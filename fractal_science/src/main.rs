use rusty_fractals::fractal::FractalCalculationType::StaticImageNebula;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem_euler::MemEuler;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, Nothing};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square2;
use rusty_fractals::{application, mathematician};

pub struct Science;

impl FractalMath<MemEuler> for Science {
    fn math(&self, me: &mut MemEuler, origin_re: f64, origin_im: f64) {
        me.square();
        me.plus(origin_re, origin_im);
        me.euler2();
        me.square();
        me.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = FractalConfig {
        name: "Science",
        fractal_calc_type: StaticImageNebula,
        orbits: Finite,

        iteration_min: 42,
        iteration_max: 8000,
        resolution_multiplier: Square2,

        palette: BlueToWhiteCircleUp,
        palette_zero: Nothing,

        width_xl: 400,
        width_xp: 401,
        height_yl: 400,
        height_yp: 401,

        width_re: 3.0,
        center_re: -0.5,
        center_im: 0.0,

        update_max: 150,
        update_min: 0,
    };

    mathematician::init_primes(fractal_config.iteration_max);
    mathematician::init_fibonacci(fractal_config.iteration_max);
    application::execute(fractal_config, Science {});
}

#[cfg(test)]
mod tests {
    use crate::Science;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem_euler::MemEuler;

    #[test]
    fn test_math() {
        let fractal = Science;
        let mut m = MemEuler::new(0.0, 0.0);

        fractal.math(&mut m, 1.0, 0.1);
        assert_eq!(m.it, 1);
    }
}
