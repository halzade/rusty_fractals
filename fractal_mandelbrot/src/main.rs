use rusty_fractals::fractal::CalculationType::InfiniteVideoZoom;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::{
    palette_blue_to_white_circle_up, palette_gray_to_black_circle_down,
};
use rusty_fractals::{machine, window};
use std::thread;

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
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_max: 2500,
        palette: palette_blue_to_white_circle_up(),
        palette_zero: palette_gray_to_black_circle_down(),

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
    // TODO

    let mandelbrot: Mandelbrot = Mandelbrot {};

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
    #[test]
    fn test_it() {}
}
