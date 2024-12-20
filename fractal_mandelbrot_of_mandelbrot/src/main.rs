use rusty_fractals::application;
use rusty_fractals::fractal::CalculationType::StaticImage;
use rusty_fractals::fractal::FractalType::MandelbrotType;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, GrayToBlackCircleDown};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;

pub struct MandelbrotOfMandelbrot {}

impl FractalMath<Mem> for MandelbrotOfMandelbrot {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        let r = m.re;
        let i = m.im;
        let or = origin_re;
        let oi = origin_im;
        // f(f(z)) : f = z^2 + c
        m.re = r * r * r * r - 6.0 * r * r * i * i + i * i * i * i + 2.0 * r * r * or
            - 2.0 * i * i * or
            - 4.0 * r * i * oi
            + or * or
            - oi * oi
            + or
            - r;
        m.im = 4.0 * r * r * r * i - 4.0 * r * i * i * i + 4.0 * r * i * or + 2.0 * r * r * oi
            - 2.0 * i * i * oi
            + 2.0 * or * oi
            + oi
            - i;
    }
}

fn main() {
    let fractal_config = FractalConfig {
        name: "Mandelbrot of Mandelbrot",
        fractal_type: MandelbrotType,
        iteration_min: 0,
        iteration_max: 2500,
        resolution_multiplier: Single,

        palette: BlueToWhiteCircleUp,
        palette_zero: GrayToBlackCircleDown,

        width_x: 1280,
        height_y: 720,
        width_re: 3.5,
        center_re: -0.5,
        center_im: 0.0,

        calc_type: StaticImage,
        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };

    application::init(fractal_config, MandelbrotOfMandelbrot {}).execute();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
