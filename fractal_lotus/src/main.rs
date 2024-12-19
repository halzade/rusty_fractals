use rusty_fractals::application;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::palette_blue_to_white_circle_up;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square11;

pub struct Lotus {}

impl FractalMath<Mem> for Lotus {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config: FractalConfig = FractalConfig {
        name: "Lotus",
        iteration_min: 42,
        iteration_max: 8000,
        resolution_multiplier: Square11,
        palette: palette_blue_to_white_circle_up(),

        width_x: 1280,
        height_y: 1000,
        width_re: 3.5,
        center_re: 0.0, //  0.67748277351478,
        center_im: 0.0, // -1.18770078111202,

        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };

    let fractal = Lotus {};

    // start program window
    let application = application::init(fractal_config);

    // execute calculation
    application.calculate(&fractal);
}

#[cfg(test)]
mod tests {
    use crate::Lotus;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let lotus = Lotus {};
        let mut m = Mem { re: 0.0, im: 0.0 };
        lotus.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
