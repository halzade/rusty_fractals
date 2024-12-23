mod euler;
mod mem_euler;
mod pixel;

use crate::mem_euler::MemEuler;
use rusty_fractals::application;
use rusty_fractals::data_image::DataType::Static;
use rusty_fractals::fractal::CalculationType::StaticImage;
use rusty_fractals::fractal::FractalType::NebulaEulerType;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, Nothing};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;

struct Euler {}

impl FractalMath<MemEuler> for Euler {
    fn math(&self, me: &mut MemEuler, origin_re: f64, origin_im: f64) {
        me.square();
        me.plus(origin_re, origin_im);
        me.euler();
        me.square();
        me.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = FractalConfig {
        name: "Euler",
        iteration_min: 42,
        iteration_max: 80000,
        fractal_type: NebulaEulerType,
        resolution_multiplier: Single,
        palette: BlueToWhiteCircleUp, // TODO PALETTE_3_RGB
        palette_zero: Nothing,

        // area
        width_x: 1920,
        height_y: 1080,
        width_re: 4.0,
        center_re: 0.0,
        center_im: 0.0,

        // calculation config
        calc_type: StaticImage,
        data_image_type: Static,
        orbits: Infinite, // ?
        update_max: 150,
        update_min: 0,
    };

    application::execute(fractal_config, Euler {});
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
