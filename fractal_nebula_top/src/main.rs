use rusty_fractals::application;
use rusty_fractals::fractal::CalculationType::InfiniteVideoZoom;
use rusty_fractals::fractal::FractalType::NebulaType;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{Nothing, PurpleToWhite};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square2;

pub struct NebulaTop {}

impl FractalMath<Mem> for NebulaTop {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = FractalConfig {
        name: "Nebula Top",
        fractal_type: NebulaType,
        iteration_min: 42,
        iteration_max: 14800,
        resolution_multiplier: Square2,

        palette: PurpleToWhite,
        palette_zero: Nothing,

        width_x: 1280,
        height_y: 720,
        width_re: 6.0,
        center_re: -1.40115859004747,
        center_im: -0.00000000709356,

        calc_type: InfiniteVideoZoom,
        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };

    application::init(fractal_config, NebulaTop {}).execute();
}

#[cfg(test)]
mod tests {
    use crate::NebulaTop;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let nebula = NebulaTop {};
        let mut m = Mem { re: 0.0, im: 0.0 };

        nebula.math(&mut m, 1.0, 0.1);

        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
