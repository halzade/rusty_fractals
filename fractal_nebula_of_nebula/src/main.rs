use rusty_fractals_common::area::AreaConfig;
use rusty_fractals_common::calc::CalculationConfig;
use rusty_fractals_common::calc::OrbitType::Finite;
use rusty_fractals_common::fractal::{FractalConfig, FractalMath};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square3;
use rusty_fractals_core::application::Application;
use rusty_fractals_core::{application, window};
use std::thread;

pub struct NebulaOfNebula<'lt> {
    app: Application<'lt>,
}

impl FractalMath<Mem> for NebulaOfNebula<'_> {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        let r = m.re;
        let i = m.im;
        let or = origin_re;
        let oi = origin_im;
        // f(f(z)) : f = z^2 + c
        m.re = r * r * r * r - 6.0 * r * r * i * i + i * i * i * i + 2.0 * r * r * or
            - 2.0 * i * i * or
            - 4.0 * r * i * oi
            + or * or
            - oi * oi
            + or
            - r;
        m.im = 4.0 * r * r * r * i - 4.0 * r * i * i * i + 4.0 * r * i * or + 2.0 * r * r * oi
            - 2.0 * i * i * oi
            + 2.0 * or * oi
            + oi
            - i;
    }
}

fn main() {
    let name: &'static str = "Nebula of Nebula";
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_min: 42,
        iteration_max: 24800,
        resolution_multiplier: Square3,
        palette: palette_blue_to_white_circle_up(),
        phantom: Default::default(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 1000,
        width_re: 0.5,
        center_re: 0.0,
        center_im: 0.0,
    };
    let calculation_config = CalculationConfig {
        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut fractal: NebulaOfNebula<'static> = NebulaOfNebula { app: application };
    let app = window::show(&fractal);
    thread::spawn(move || {
        // TODO fractal.calculate_fractal();
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::NebulaOfNebula;
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_core::application;

    #[test]
    fn test_math() {
        let nebula = NebulaOfNebula {
            app: application::init_none(),
        };
        let mut m = Mem { re: 0.0, im: 0.0 };
        nebula.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
