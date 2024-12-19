use rusty_fractals::area::AreaConfig;
use rusty_fractals::calc::CalculationConfig;
use rusty_fractals::calc::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::palette_black_to_white_circle_up;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square9;
use rusty_fractals::{machine, window};
use std::thread;

pub struct Nebula<'lt> {
    app: Application<'lt>,
}

impl FractalMath<Mem> for Nebula<'_> {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_min: 42,
        iteration_max: 14800,
        resolution_multiplier: Square9,
        palette: palette_black_to_white_circle_up(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 1000,
        width_re: 3.5,
        center_re: -0.5,
        center_im: 0.0,
    };
    let calculation_config = CalculationConfig {
        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut fractal: Nebula<'static> = Nebula { app: application };
    let app = window::show(&fractal);
    thread::spawn(move || {
        // TODO fractal.calculate_fractal();
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::Nebula;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;
    use rusty_fractals::application;

    #[test]
    fn test_math() {
        let nebula = Nebula {
            app: application::init_none(),
        };
        let mut m = Mem { re: 0.0, im: 0.0 };
        nebula.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
