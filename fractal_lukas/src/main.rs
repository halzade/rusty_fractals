use PaletteName::BlueToWhiteCircleUp;
use rusty_fractals::infra::config::NebulaImage;
use rusty_fractals::rusty::fractal::FractalMath;
use rusty_fractals::rusty::fractal::OrbitType::Infinite;
use rusty_fractals::calc::mem::Mem;
use rusty_fractals::color::palettes::PaletteName;
use rusty_fractals::domain::resolution_multiplier::ResolutionMultiplier::Square3;
use rusty_fractals::rusty::application;
use rusty_fractals::calc::mathematician;

struct Lukas {}

impl FractalMath<Mem> for Lukas {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);

        m.lukas();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = NebulaImage {
        name: "Lukas",
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

    mathematician::init_happy(fractal_config.iteration_max);
    application::execute(fractal_config.init(), Lukas {});
}

#[cfg(test)]
mod tests {
    use crate::Lukas;
    use rusty_fractals::rusty::fractal::{FractalMath, MemType};
    use rusty_fractals::calc::mathematician;
    use rusty_fractals::calc::mem::Mem;

    #[tokio::test]
    async fn test_math() {
        let lukas = Lukas {};
        let mut m = Mem::new(0.5, 0.5);
        mathematician::init_happy(2);

        lukas.math(&mut m, 1.0, 0.0);
        assert_eq!(m.re, 0.9997);
        assert_eq!(m.im, 0.0004);
    }
}
