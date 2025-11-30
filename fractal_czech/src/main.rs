use PaletteName::BlueToWhiteCircleUp;
use rusty_fractals::config::NebulaImage;
use rusty_fractals::fractal::FractalMath;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square3;
use rusty_fractals::{application, mathematician};

struct Czech {}

impl FractalMath<Mem> for Czech {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);

        m.czech();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = NebulaImage {
        name: "Czech",
        iteration_min: 42,
        iteration_max: 8000,
        resolution_multiplier: Square3,
        palette: BlueToWhiteCircleUp,

        width_x: 1000,
        height_y: 1000,
        width_re: 3.0,
        center_re: -0.5,
        center_im: 0.0,
        orbits: Infinite,
    };

    mathematician::init_perfect(fractal_config.iteration_max);
    application::execute(fractal_config.init(), Czech {});
}

#[cfg(test)]
mod tests {
    use crate::Czech;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mathematician;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let czech = Czech {};
        let mut m = Mem::new(0.0, 0.0);

        mathematician::init_perfect(2);

        czech.math(&mut m, 1.0, 0.0);
        assert_eq!(m.re, 2.00);
        assert_eq!(m.im, 0.0);
    }
}
