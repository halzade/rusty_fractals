use rusty_fractals::rusty::application;
use rusty_fractals::infra::config::NebulaImage;
use rusty_fractals::rusty::fractal::FractalMath;
use rusty_fractals::rusty::fractal::OrbitType::Infinite;
use rusty_fractals::calc::mem::Mem;
use rusty_fractals::color::palettes::PaletteName::BlueToWhiteCircleUp;
use rusty_fractals::domain::resolution_multiplier::ResolutionMultiplier::Single;

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
    let fractal_config = NebulaImage {
        name: "Fat Star",

        iteration_min: 8,
        iteration_max: 81000,
        resolution_multiplier: Single,
        palette: BlueToWhiteCircleUp,

        width_x: 1200,
        height_y: 1200,
        width_re: 0.15,
        center_re: 0.5425,
        center_im: -0.31,

        orbits: Infinite,
    };

    application::execute(fractal_config.init(), FatStarMagnific {});
}

#[cfg(test)]
mod tests {
    use crate::FatStarMagnific;
    use rusty_fractals::rusty::fractal::{FractalMath, MemType};
    use rusty_fractals::calc::mem::Mem;

    #[tokio::test]
    async fn test_math() {
        let fat_star_magnific = FatStarMagnific {};
        let mut m = Mem::new(0.0, 0.0);

        fat_star_magnific.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
