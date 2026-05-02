use rusty_fractals::rusty::application;
use rusty_fractals::infra::config::NebulaImage;
use rusty_fractals::rusty::fractal::FractalMath;
use rusty_fractals::rusty::fractal::OrbitType::Finite;
use rusty_fractals::calc::mem::Mem;
use rusty_fractals::color::palettes::PaletteName::BlueToWhiteCircleUp;
use rusty_fractals::domain::resolution_multiplier::ResolutionMultiplier::Square5;

pub struct Lotus {}

impl FractalMath<Mem> for Lotus {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = NebulaImage {
        name: "Lotus",

        iteration_min: 42,
        iteration_max: 8000,
        resolution_multiplier: Square5,

        palette: BlueToWhiteCircleUp,

        width_x: 1280,
        height_y: 1000,
        width_re: 3.5,
        center_re: 0.0,
        center_im: 0.0,
        // video
        // center_re: 0.67748277351478,
        // center_im: -1.18770078111202,
        orbits: Finite,
    };

    application::execute(fractal_config.init(), Lotus {});
}

#[cfg(test)]
mod tests {
    use crate::Lotus;
    use rusty_fractals::rusty::fractal::{FractalMath, MemType};
    use rusty_fractals::calc::mem::Mem;

    #[tokio::test]
    async fn test_math() {
        let lotus = Lotus {};
        let mut m = Mem::new(0.0, 0.0);

        lotus.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
