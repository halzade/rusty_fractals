use rusty_fractals_common::area::{AreaConfig};
use rusty_fractals_common::calc::CalculationConfig;
use rusty_fractals_common::calc::OrbitType::Finite;
use rusty_fractals_common::fractal::{FractalConfig, FractalMath};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square11;
use rusty_fractals_core::application::Application;
use rusty_fractals_core::{application, window};
use std::thread;

pub struct Lotus<'lt> {
    app: Application<'lt>,
}

impl FractalMath<Mem> for Lotus<'_> {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_min: 42,
        iteration_max: 8000,
        resolution_multiplier: Square11,
        palette: palette_blue_to_white_circle_up(),
        phantom: Default::default(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 1000,
        width_re: 3.5,
        center_re: 0.0, //  0.67748277351478,
        center_im: 0.0, // -1.18770078111202,
    };
    let calculation_config = CalculationConfig {
        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut fractal: Lotus<'static> = Lotus { app: application };
    let app = window::show(&fractal);
    thread::spawn(move || {
        // TODO fractal.calculate_fractal();
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::Lotus;
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_core::application;

    #[test]
    fn test_math() {
        let lotus = Lotus {
            app: application::init_none(),
        };
        let mut m = Mem { re: 0.0, im: 0.0 };
        lotus.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
