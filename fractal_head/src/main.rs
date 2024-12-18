use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::calc::CalculationConfig;
use rusty_fractals_common::calc::OrbitType::Finite;
use rusty_fractals_common::constants::{PHOENIX_INIT_C, PHOENIX_INIT_P};
use rusty_fractals_common::fractal::{FractalConfig, FractalMath};
use rusty_fractals_common::mem_phoenix::MemPhoenix;
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square9;
use rusty_fractals_core::application::Application;
use rusty_fractals_core::{application, machine, window};
use std::thread;

pub struct Head<'lt> {
    name: &'static str,
    app: Application<'lt>,
}

impl FractalMath<MemPhoenix> for Head<'_> {
    fn math(&self, mp: &mut MemPhoenix, origin_re: f64, origin_im: f64) {
        mp.square();
        mp.m.re += PHOENIX_INIT_C;
        mp.m.re += PHOENIX_INIT_P * mp.prev_prev_re;
        mp.m.im += PHOENIX_INIT_P * mp.prev_prev_im;
        // previous iteration values
        mp.prev_prev_re = mp.prev_re;
        mp.prev_prev_im = mp.prev_im;
        mp.prev_re = mp.m.re;
        mp.prev_im = mp.m.im;
        mp.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = FractalConfig {
        iteration_min: 8,
        iteration_max: 25000,
        resolution_multiplier: Square9,
        palette: palette_blue_to_white_circle_up(),
        phantom: Default::default(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 720,
        width_re: 5.0,
        center_re: -0.16884290496519,
        center_im: -0.37573460559804,
    };
    let calculation_config = CalculationConfig {
        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut head: Head<'static> = Head {
        name: "Head",
        app: application,
    };
    let app = window::show(&head);
    thread::spawn(move || {

        // TODO head.calculate_fractal();
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::Head;
    use rusty_fractals_common::constants::PHOENIX_INITIALIZER;
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_common::mem_phoenix::MemPhoenix;
    use rusty_fractals_core::application;

    #[test]
    fn test_math() {
        let head = Head {
            name: "Head",
            app: application::init_none(),
        };
        let mut mp = MemPhoenix {
            m: Mem { re: 0.0, im: 0.0 },
            prev_prev_re: PHOENIX_INITIALIZER,
            prev_prev_im: PHOENIX_INITIALIZER,
            prev_re: PHOENIX_INITIALIZER,
            prev_im: PHOENIX_INITIALIZER,
        };
        head.math(&mut mp, 1.0, 0.1);
        assert_eq!(mp.re(), 1.1);
        assert_eq!(mp.im(), -0.15);
    }
}
