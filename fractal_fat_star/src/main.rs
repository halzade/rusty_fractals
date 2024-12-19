use rusty_fractals::area::AreaConfig;
use rusty_fractals::calc::CalculationConfig;
use rusty_fractals::calc::OrbitType::Infinite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::palette_blue_to_white_circle_up;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals::{window};
use rusty_fractals::calc::CalculationType::StaticImage;
use rusty_fractals::fractal;

pub struct FatStar<'lt> {}

impl FractalMath<Mem> for FatStar<'_> {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_min: 42,
        iteration_max: 22000,
        resolution_multiplier: Single,
        palette: palette_blue_to_white_circle_up(),
    };
    let area_config = AreaConfig {
        width_x: 800,
        height_y: 800,
        width_re: 3.5,
        center_re: 0.0,
        center_im: 0.0,
    };
    let calculation_config = CalculationConfig {
        calc_type: StaticImage,
        orbits: Infinite,
        update_max: 150,
        update_min: 0,
    };

    let fat_star: FatStar<'static> = FatStar {};
    let app = window::show("fat_star");

    fractal::calculate_fractal_new_thread(&fat_star, fractal_config, area_config, calculation_config);

    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::FatStar;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;
    use rusty_fractals::application;

    #[test]
    fn test_math() {
        let fat_star: FatStar<'static> = FatStar {
            app: application::init_trivial(),
        };
        let mut m = Mem { re: 0.0, im: 0.0 };
        fat_star.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
