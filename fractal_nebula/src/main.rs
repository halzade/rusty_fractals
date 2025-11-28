use rusty_fractals::application;
use rusty_fractals::config::NebulaImage;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalMath, Optimizer};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::BlackToWhiteCircleUp;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square9;

pub struct Nebula {}

impl FractalMath<Mem> for Nebula {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = NebulaImage {
        name: "Nebula",

        iteration_min: 42,
        iteration_max: 14800,
        resolution_multiplier: Square9,
        palette: BlackToWhiteCircleUp,

        width_x: 1280,
        height_y: 1000,
        width_re: 3.5,
        center_re: -0.5,
        center_im: 0.0,

        orbits: Finite,
    };

    let o = Optimizer::nebula_optimization();

    application::execute_o(fractal_config.init(), Nebula {}, Some(o));
}

#[cfg(test)]
mod tests {
    use crate::Nebula;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let nebula = Nebula {};
        let mut m = Mem::new(0.0, 0.0);

        nebula.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
