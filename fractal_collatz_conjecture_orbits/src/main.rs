use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem_collatz::MemCollatz;
use rusty_fractals::palettes::palette_blue_to_white_circle_up;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square11;
use rusty_fractals::{machine, window};
use std::thread;

pub struct CollatzConjectureOrbits<'lt> {
    app: Application<'lt>,
}

impl FractalMath<MemCollatz> for CollatzConjectureOrbits<'_> {
    fn math(&self, m: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        m.square();
        m.collatz_conjecture();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let name: &'static str = "CollatzConjectureOrbits";
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_min: 7,
        iteration_max: 1348,
        resolution_multiplier: Square11,
        palette: palette_blue_to_white_circle_up(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 720,
        width_re: 5.0,
        center_re: -0.088485445553580480,
        center_im: -0.200679435068532800,
    };
    let calculation_config = CalculationConfig {
        orbits: Infinite,
        update_max: 150,
        update_min: 0,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut fractal: CollatzConjectureOrbits<'static> =
        CollatzConjectureOrbits { app: application };
    let app = window::show(&fractal);
    thread::spawn(move || {
        // TODO fractal.calculate_fractal();
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::CollatzConjectureOrbits;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;
    use rusty_fractals::mem_collatz::MemCollatz;
    use rusty_fractals::application;

    #[test]
    fn test_math() {
        let collatz = CollatzConjectureOrbits {
            app: application::init_none(),
        };
        let mut mc = MemCollatz {
            m: Mem { re: 0.0, im: 0.0 },
            num: 7,
        };
        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.re(), 2.0);
        assert_eq!(mc.im(), 1.1);
    }
}
