use rusty_fractals::application;
use rusty_fractals::data_image::DataType::Static;
use rusty_fractals::fractal::CalculationType::StaticImage;
use rusty_fractals::fractal::FractalType::NebulaType;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, Nothing};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::{Single, Square2, Square5};

pub struct Science;

impl FractalMath for Science {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = FractalConfig {
        name: "Science",
        fractal_type: NebulaType,
        iteration_min: 42,
        iteration_max: 800,
        resolution_multiplier: Square5,

        palette: BlueToWhiteCircleUp,
        palette_zero: Nothing,

        width_x: 600,
        height_y: 600,
        width_re: 3.0,
        center_re: -0.5,
        center_im: 0.0,

        calc_type: StaticImage,
        data_image_type: Static,

        // calc_type: InfiniteVideoZoom,
        // data_image_type: Dynamic,

        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };

    application::execute(fractal_config, Science {});
}

#[cfg(test)]
mod tests {
    use crate::Science;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let fractal = Science;
        let mut m = Mem { re: 0.0, im: 0.0 };

        fractal.math(&mut m, 1.0, 0.1);

        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
