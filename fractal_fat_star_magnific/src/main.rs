use rusty_fractals::application;
use rusty_fractals::fractal::CalculationType::StaticImage;
use rusty_fractals::fractal::FractalType::NebulaType;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, Nothing};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;

pub struct FatStarMagnific {}

impl FractalMath<Mem> for FatStarMagnific {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = FractalConfig {
        name: "Fat Star",
        iteration_min: 8,
        iteration_max: 81000,
        fractal_type: NebulaType,
        resolution_multiplier: Single,
        palette: BlueToWhiteCircleUp,

        // TODO
        // const INIT_FINEBROT_AREA_SIZE : f64= 0.15;
        // const INIT_FINEBROT_TARGET_re : f64= 0.5425;
        // const INIT_FINEBROT_TARGET_im : f64= -0.31;
        palette_zero: Nothing,
        width_x: 600,
        height_y: 600,
        width_re: 3.0,
        center_re: 0.0,
        center_im: 0.0,

        calc_type: StaticImage,
        orbits: Infinite,
        update_max: 150,
        update_min: 0,
    };

    application::init(fractal_config, FatStarMagnific {}).execute();
}

#[cfg(test)]
mod tests {
    use crate::FatStarMagnific;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let fat_star_magnific = FatStarMagnific {};
        let mut m = Mem { re: 0.0, im: 0.0 };

        fat_star_magnific.math(&mut m, 1.0, 0.1);

        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
