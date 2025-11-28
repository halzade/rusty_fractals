use rusty_fractals::application;
use rusty_fractals::config::NebulaImage;
use rusty_fractals::fractal::FractalMath;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::BlueToWhiteCircleUp;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;

pub struct InfinityTop {}

impl FractalMath<Mem> for InfinityTop {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = NebulaImage {
        name: "Infinity Top",

        iteration_min: 3000,
        iteration_max: 180_000,
        resolution_multiplier: Single,

        palette: BlueToWhiteCircleUp,

        width_x: 600,
        height_y: 600,
        width_re: 1.8,
        center_re: -1.0,
        center_im: 0.0,

        orbits: Infinite,
    };

    application::execute(fractal_config.init(), InfinityTop {});
}

#[cfg(test)]
mod tests {
    use crate::InfinityTop;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let infinity = InfinityTop {};
        let mut m = Mem::new(0.0, 0.0);

        infinity.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
