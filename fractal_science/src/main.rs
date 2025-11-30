use rusty_fractals::config::NebulaImage;
use rusty_fractals::fractal::FractalMath;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::BlueToWhiteCircleUp;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square2;
use rusty_fractals::{application, mathematician};

pub struct Science;

impl FractalMath<Mem> for Science {
    fn math(&self, me: &mut Mem, origin_re: f64, origin_im: f64) {
        me.square();
        me.plus(origin_re, origin_im);
        me.czech();
        // me.czech();
        // me.chess();
        // me.pythagoras();
        // me.bee();
        // me.taco();
        // me.manana();
        // me.potato();
        me.square();
        me.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = NebulaImage {
        name: "Science",
        orbits: Finite,

        iteration_min: 42,
        iteration_max: 2000,
        resolution_multiplier: Square2,
        palette: BlueToWhiteCircleUp,

        width_x: 600,
        height_y: 600,
        width_re: 3.0,
        center_re: -0.5,
        center_im: 0.0,
    };

    mathematician::init_perfect(fractal_config.iteration_max);

    application::execute(fractal_config.init(), Science {});
}

#[cfg(test)]
mod tests {
    use crate::Science;
    use rusty_fractals::fractal::{FractalMath, MemType};
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let fractal = Science;
        let mut m = Mem::new(0.0, 0.0);

        fractal.math(&mut m, 1.0, 0.1);
        assert_eq!(m.it, 1);
    }
}
