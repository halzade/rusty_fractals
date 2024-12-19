use rusty_fractals::application;
use rusty_fractals::constants::{PHOENIX_INIT_C, PHOENIX_INIT_P};
use rusty_fractals::fractal::CalculationType::StaticImage;
use rusty_fractals::fractal::FractalType::NebulaType;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem_phoenix::MemPhoenix;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, Nothing};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square9;

pub struct Head {}

impl FractalMath<MemPhoenix> for Head {
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
        name: "Head",
        iteration_min: 8,
        iteration_max: 25000,
        fractal_type: NebulaType,
        resolution_multiplier: Square9,
        palette: BlueToWhiteCircleUp,

        palette_zero: Nothing,
        width_x: 1280,
        height_y: 720,
        width_re: 5.0,
        center_re: -0.16884290496519,
        center_im: -0.37573460559804,

        calc_type: StaticImage,
        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };
    let head = Head {};

    // start program window
    let application = application::init(fractal_config);

    // execute calculation
    application.calculate(&head);
}

#[cfg(test)]
mod tests {
    use crate::Head;
    use rusty_fractals::constants::PHOENIX_INITIALIZER;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;
    use rusty_fractals::mem_phoenix::MemPhoenix;

    #[test]
    fn test_math() {
        let head = Head {};
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
