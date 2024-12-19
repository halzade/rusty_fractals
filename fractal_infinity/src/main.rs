use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::palette_blue_to_white_circle_up;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals::{machine, window};
use std::thread;

pub struct Infinity<'lt> {
    app: Application<'lt>,
}

impl FractalMath<Mem> for Infinity<'_> {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let name = "Infinity";
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_min: 3000,
        iteration_max: 30_000,
        resolution_multiplier: Single,
        palette: palette_blue_to_white_circle_up(),
    };
    let area_config = AreaConfig {
        width_x: 600,
        height_y: 600,
        width_re: 2.6,
        center_re: -0.5,
        center_im: 0.0,
    };
    let calculation_config = CalculationConfig {
        orbits: Infinite,
        update_max: 150,
        update_min: 0,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut fractal: Infinity<'static> = Infinity { app: application };
    let app = window::show(&fractal);
    thread::spawn(move || {
        // TODO fractal.calculate_fractal();
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::Infinity;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;
    use rusty_fractals::application;

    #[test]
    fn test_math() {
        let infinity = Infinity {
            app: application::init_none(),
        };
        let mut m = Mem { re: 0.0, im: 0.0 };
        infinity.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
