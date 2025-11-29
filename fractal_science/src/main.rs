use rusty_fractals::fractal::FractalCalculationType::StaticImageNebula;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem_euler::MemEuler;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, Nothing};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::{Square2, Square3};
use rusty_fractals::{application, mathematician};

pub struct Science;

impl FractalMath<MemEuler> for Science {
    fn math(&self, me: &mut MemEuler, origin_re: f64, origin_im: f64) {
        me.square();
        me.plus(origin_re, origin_im);
        me.euler();
        if mathematician::is_prime(me.it) {
            mathematician::circle_inversion(&mut me.m, 1.0, 1.0);
        }
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
        resolution_multiplier: Square3,

        palette: BlueToWhiteCircleUp,
        palette_zero: Nothing,

        width_xl: 800,
        width_xp: 801,
        height_yl: 800,
        height_yp: 801,

        width_re: 3.5,
        center_re: 0.0,
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
