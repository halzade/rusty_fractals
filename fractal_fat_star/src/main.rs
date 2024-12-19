use rusty_fractals::fractal::CalculationType::StaticImage;
use rusty_fractals::fractal::FractalType::Nebula;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, Nothing};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals::{machine, window};

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
        fractal_type: Nebula,
        resolution_multiplier: Single,
        palette: BlueToWhiteCircleUp,
        palette_zero: Nothing,

        // area
        width_x: 800,
        height_y: 800,
        width_re: 3.5,
        center_re: 0.0,
        center_im: 0.0,

        // calculation config
        calc_type: StaticImage,
        orbits: Infinite,
        update_max: 150,
        update_min: 0,
    };

    // instantiate fractal
    let fat_star: FatStar<'static> = FatStar {};

    // init the calculation machinery
    let machine = machine::init("FatStar", &fractal_config);

    // start program window
    let app = window::show(&machine);

    // execute calculation
    machine.calculate(&fat_star);

    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::FatStar;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let fat_star: FatStar<'static> = FatStar {};
        let mut m = Mem { re: 0.0, im: 0.0 };
        fat_star.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
