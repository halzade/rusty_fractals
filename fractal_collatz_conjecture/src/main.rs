use std::sync::{Arc, Mutex};
use std::thread;
use rusty_fractals_core::{machine_mandelbrot, window};
use rusty_fractals_common::area::{AreaConfig};
use rusty_fractals_common::{data_image, fractal};
use rusty_fractals_common::fractal::{CalculationConfig, FractalMandelbrot, FractalMath};
use rusty_fractals_common::mem_collatz::MemCollatz;
use rusty_fractals_common::palettes::{palette_blue_to_white_circle_up, palette_gray_to_blue, ResultConfigMandelbrot};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square3;

struct CollatzConjecture {}

impl FractalMath<MemCollatz> for CollatzConjecture {
    fn math(&self, m: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        m.square();
        m.collatz_conjecture();
        m.plus(origin_re, origin_im);
    }
}

impl FractalMandelbrot for CollatzConjecture {
    fn calculate_mandelbrot_path(&self, iteration_max: u32, origin_re: f64, origin_im: f64) -> (u32, f64) {
        fractal::calculate_mandelbrot_path(self, iteration_max, origin_re, origin_im)
    }
}

fn main() {
    let name = "Collatz Conjecture";

    const WIDTH: usize = 1280;
    const HEIGHT: usize = 720;

    let calculation_config = CalculationConfig {
        iteration_min: 42,
        iteration_max: 14800,
        resolution_multiplier: Square3,
    };
    let area_config = AreaConfig {
        width_re: 7.0,
        center_re: -0.088485445553580480,
        center_im: -0.200679435068532800,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfigMandelbrot {
        palette: palette_blue_to_white_circle_up(),
        palette_zero: palette_gray_to_blue(),
    };

    let collatz = CollatzConjecture {};
    let machine = machine_mandelbrot::init(&calculation_config, result_config, &area_config);

    let data_image = data_image::init_data_image(machine.area());
    let mut app_window = window::init(name, WIDTH, HEIGHT);
    let app = app_window.show(&data_image.image_init().as_raw(), WIDTH, HEIGHT);
    let mutex_window = Arc::new(Mutex::new(app_window));

    thread::spawn(move || {
        machine.calculate_mandelbrot(&collatz, &data_image, mutex_window);
    });
    app.run().unwrap();
}


#[cfg(test)]
mod tests {
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_common::mem_collatz::MemCollatz;
    use crate::CollatzConjecture;

    #[test]
    fn test_math() {
        let collatz = CollatzConjecture {};
        let mut mc = MemCollatz { m: Mem { re: 0.0, im: 0.0 }, it: 1 };
        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.re(), 2.0);
        assert_eq!(mc.im(), 1.1);
    }
}