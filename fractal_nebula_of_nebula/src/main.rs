use rusty_fractals::application;
use rusty_fractals::fractal::CalculationType::StaticImage;
use rusty_fractals::fractal::FractalType::NebulaType;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, Nothing};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square3;

pub struct NebulaOfNebula {}

impl FractalMath<Mem> for NebulaOfNebula {
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
    let fractal_config = FractalConfig {
        name: "Nebula of Nebula",
        fractal_type: NebulaType,
        iteration_min: 42,
        iteration_max: 24800,
        resolution_multiplier: Square3,

        palette: BlueToWhiteCircleUp,
        palette_zero: Nothing,

        width_x: 1280,
        height_y: 1000,
        width_re: 0.5,
        center_re: 0.0,
        center_im: 0.0,

        calc_type: StaticImage,
        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };

    application::init(fractal_config, NebulaOfNebula {}).execute();
}

#[cfg(test)]
mod tests {
    use crate::NebulaOfNebula;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let nebula_of_nebula = NebulaOfNebula {};
        let mut m = Mem { re: 0.0, im: 0.0 };
        nebula_of_nebula.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
