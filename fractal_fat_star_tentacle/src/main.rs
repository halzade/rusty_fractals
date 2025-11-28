use rusty_fractals::application;
use rusty_fractals::config::NebulaImage;
use rusty_fractals::fractal::FractalMath;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::BlueToWhiteCircleUp;
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
    let fractal_config = NebulaImage {
        name: "Fat Star Tentacle",

        iteration_min: 42,
        iteration_max: 2200,
        resolution_multiplier: Single,
        palette: BlueToWhiteCircleUp,

        width_x: 600,
        height_y: 600,
        width_re: 0.5,
        center_re: 0.5,
        center_im: -0.38,

        orbits: Infinite,
    };

    application::execute(fractal_config.init(), FatStarTentacle {});
}

#[cfg(test)]
mod tests {
    use crate::FatStarTentacle;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let fat_star = FatStarTentacle {};
        let mut m = Mem::new(0.0, 0.0);

        fat_star.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
