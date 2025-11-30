use rusty_fractals::config::EulerImage;
use rusty_fractals::fractal::FractalMath;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::mem::Mem;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals::{application, mathematician};

/** Fractal Euler type uses three color spectra for better mathematical analysis and better coloring results.
 *  Possible use as:
 *  - prime path length & el. order      -> Red spectrum
 *  - Fibonacci path lengths & el. order -> Green spectrum
 *  - other path lengths & el. order     -> Blue spectrum
 */

struct Euler {}

impl FractalMath<Mem> for Euler {
    fn math(&self, me: &mut Mem, origin_re: f64, origin_im: f64) {
        me.square();
        me.plus(origin_re, origin_im);
        me.euler();
        me.square();
        me.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = EulerImage {
        name: "Euler",

        iteration_min: 42,
        iteration_max: 80000,
        resolution_multiplier: Single,

        // area
        width_x: 400,
        height_y: 400,
        width_re: 4.0,
        center_re: 0.0,
        center_im: 0.0,

        // calculation config
        orbits: Infinite,
    };

    mathematician::init_primes(fractal_config.iteration_max);
    application::execute(fractal_config.init(), Euler {});
}

#[cfg(test)]
mod tests {
    use crate::Euler;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let euler = Euler {};
        let mut m = Mem::new(0.0, 0.0);

        euler.math(&mut m, 1.0, 0.0);
        assert_eq!(m.re, 2.00);
        assert_eq!(m.im, 0.0);
    }
}
