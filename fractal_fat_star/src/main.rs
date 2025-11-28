use rusty_fractals::application;
use rusty_fractals::config::NebulaImage;
use rusty_fractals::fractal::FractalMath;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::BlueToWhiteCircleUp;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;

pub struct FatStar {}

impl FractalMath<Mem> for FatStar {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = NebulaImage {
        name: "Fat Star",

        iteration_min: 42,
        iteration_max: 22000,
        resolution_multiplier: Single,
        palette: BlueToWhiteCircleUp,

        // area
        width_x: 800,
        height_y: 800,
        width_re: 3.5,
        center_re: 0.0,
        center_im: 0.0,

        // calculation config
        orbits: Infinite,
    };

    application::execute(fractal_config.init(), FatStar {});
}

#[cfg(test)]
mod tests {
    use crate::FatStar;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let fat_star: FatStar = FatStar {};
        let mut m = Mem::new(0.0, 0.0);

        fat_star.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
