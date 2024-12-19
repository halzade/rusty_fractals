use rusty_fractals::application;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::palette_blue_to_white_circle_up;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;

pub struct FatStarTentacle {}

impl FractalMath<Mem> for FatStarTentacle {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config: FractalConfig = FractalConfig {
        name: "Fat Star Tentacle",
        iteration_min: 42,
        iteration_max: 22000,
        resolution_multiplier: Single,
        palette: palette_blue_to_white_circle_up(),

        // TODO
        // const INIT_FINEBROT_AREA_SIZE : f64= 0.5;
        // const INIT_FINEBROT_TARGET_re : f64= 0.5;
        // const INIT_FINEBROT_TARGET_im : f64= -0.38;
        width_x: 600,
        height_y: 600,
        width_re: 3.5,
        center_re: 0.0,
        center_im: 0.0,

        orbits: Infinite,
        update_max: 150,
        update_min: 0,
    };

    let fat_star_tentacle = FatStarTentacle {};

    // start program window
    let application = application::init(fractal_config);

    // execute calculation
    application.calculate(&fat_star_tentacle);
}

#[cfg(test)]
mod tests {
    use crate::application;
    use crate::FatStarTentacle;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let fat_star = FatStarTentacle {};
        let mut m = Mem { re: 0.0, im: 0.0 };
        fat_star.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
