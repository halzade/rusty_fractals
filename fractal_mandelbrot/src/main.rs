use rusty_fractals::application;
use rusty_fractals::fractal::CalculationType::InfiniteVideoZoom;
use rusty_fractals::fractal::FractalType::MandelbrotType;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::PaletteName::{BlueToWhiteCircleUp, GrayToBlackCircleDown};
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;

/**
 * The Mandelbrot Fractal
 */
pub struct Mandelbrot {}

/**
 * x := x^2 + y^2 + x0
 * y := 2xy + y0
 */
impl FractalMath<Mem> for Mandelbrot {
    fn math(&self, mc: &mut Mem, origin_re: f64, origin_im: f64) {
        mc.square();
        mc.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config = FractalConfig {
        name: "Mandelbrot",
        fractal_type: MandelbrotType,
        iteration_min: 0,
        iteration_max: 2500,
        resolution_multiplier: Single,

        palette: BlueToWhiteCircleUp,
        palette_zero: GrayToBlackCircleDown,

        width_x: 1280,
        height_y: 720,
        width_re: 4.5,
        center_re: -0.5,
        center_im: 0.0,

        calc_type: InfiniteVideoZoom,
        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };

    let mandelbrot = Mandelbrot {};

    // start program window
    let application = application::init(fractal_config);

    // execute calculation
    application.calculate(&mandelbrot);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
