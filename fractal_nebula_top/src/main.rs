use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::palette_purple_to_white;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square2;
use rusty_fractals::{machine, window};
use std::thread;

pub struct NebulaTop<'lt> {

}

impl FractalMath<Mem> for NebulaTop<'_> {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_min: 42,
        iteration_max: 14800,
        resolution_multiplier: Square2,
        palette: palette_purple_to_white(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 720,
        width_re: 6.0,
        center_re: -1.40115859004747,
        center_im: -0.00000000709356,
    };
    let calculation_config = CalculationConfig {
        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut fractal: NebulaTop<'static> = NebulaTop { app: application };
    let app = window::show(&fractal);
    thread::spawn(move || {
        // TODO fractal.calculate_fractal();
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::NebulaTop;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;
    use rusty_fractals::application;

    #[test]
    fn test_math() {
        let nebula = NebulaTop {
            app: application::init_none(),
        };
        let mut m = Mem { re: 0.0, im: 0.0 };
        nebula.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
