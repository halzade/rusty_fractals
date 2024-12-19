use rusty_fractals::area::AreaConfig;
use rusty_fractals::calc::CalculationConfig;
use rusty_fractals::calc::OrbitType::Infinite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::palette_blue_to_white_circle_up;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals::application::Application;
use rusty_fractals::{application, window};
use std::thread;

pub struct FatStarTentacle<'lt> {}

impl FractalMath<Mem> for FatStarTentacle<'_> {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let name: &'static str = "Fat Star Tentacle";
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_min: 42,
        iteration_max: 22000,
        resolution_multiplier: Single,
        palette: palette_blue_to_white_circle_up(),
    };
    // TODO
    // const INIT_FINEBROT_AREA_SIZE : f64= 0.5;
    // const INIT_FINEBROT_TARGET_re : f64= 0.5;
    // const INIT_FINEBROT_TARGET_im : f64= -0.38;
    let area_config = AreaConfig {
        width_x: 600,
        height_y: 600,
        width_re: 3.5,
        center_re: 0.0,
        center_im: 0.0,
    };
    let calculation_config = CalculationConfig {
        orbits: Infinite,
        update_max: 150,
        update_min: 0,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut fractal: FatStarTentacle<'static> = FatStarTentacle { app: application };
    let app = window::show(&fractal);
    thread::spawn(move || {
        // TODO fractal.calculate_fractal();
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::application;
    use crate::FatStarTentacle;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let fat_star: FatStarTentacle<'static> = FatStarTentacle {};
        let mut m = Mem { re: 0.0, im: 0.0 };
        fat_star.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
