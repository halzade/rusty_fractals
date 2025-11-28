use rusty_fractals::application;
use rusty_fractals::config::EulerImage;
use rusty_fractals::fractal::FractalMath;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::mem_euler::MemEuler;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;

/** Fractal Euler type uses three color spectra for better mathematical analysis and better coloring results.
 *  Possible use as:
 *  - prime path length & el. order      -> Red spectrum
 *  - Fibonacci path lengths & el. order -> Green spectrum
 *  - other path lengths & el. order     -> Blue spectrum
 */

struct Euler {}

impl FractalMath<MemEuler> for Euler {
    fn math(&self, me: &mut MemEuler, origin_re: f64, origin_im: f64) {
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

    application::execute(fractal_config.init(), Euler {});
}

#[cfg(test)]
mod tests {
    use crate::Euler;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem_euler::MemEuler;

    #[test]
    fn test_math() {
        let euler = Euler {};
        let mut me = MemEuler::new(0.0, 0.0);

        euler.math(&mut me, 1.0, 0.0);
        assert_eq!(me.m.re, 2.00);
        assert_eq!(me.m.im, 0.0);
    }
}
