use std::sync::Mutex;
use std::thread;
use rusty_fractals_core::{application, machine_mandelbrot, window};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{FractalCommon, FractalMandelbrotCommon, FractalMath, MandelbrotConfig};
use rusty_fractals_common::mem_collatz::MemCollatz;
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::palettes::{palette_blue_to_white_circle_up, palette_gray_to_black_circle_down};
use rusty_fractals_core::application::Application;

pub struct CollatzConjecture<'lt> {
    app: Application<'lt>,
}

impl FractalMath<MemCollatz> for CollatzConjecture<'_> {
    fn math(&self, m: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        m.square();
        m.collatz_conjecture();
        m.plus(origin_re, origin_im);
    }
}

impl FractalMandelbrotCommon for CollatzConjecture<'_> {
    fn calculate_path(&self, iteration_max: u32, origin_re: f64, origin_im: f64) -> (u32, f64) {
        fractal::calculate_mandelbrot_path(self, iteration_max, origin_re, origin_im)
    }
    fn calculate_mandelbrot(&mut self) {
        let fm = machine_mandelbrot::init();
        fm.calculate_mandelbrot(self);
    }
    fn palette_zero(&self) -> &Palette { &self.app.palette_zero }
}

impl FractalCommon for CollatzConjecture<'_> {
    fn name(&self) -> &'static str { "Collatz Conjecture" }
    fn update(&mut self) {
        self.app.max += 150;
        println!("iteration_max = {}", self.app.max);
    }
    fn move_zoom_recalculate(&mut self, x: usize, y: usize) {
        self.app.move_target(x, y);
        self.app.zoom_in_recalculate_pixel_positions(true);
        self.calculate_mandelbrot_new_thread(&FRACTAL);
    }
    fn move_target_zoom_in_recalculate(x: usize, y: usize) {
        FRACTAL.lock().unwrap().as_mut().unwrap().move_zoom_recalculate(x, y);
    }
}

pub static FRACTAL: Mutex<Option<CollatzConjecture>> = Mutex::new(None);

fn main() {
    let mandelbrot_config: MandelbrotConfig<'static> = MandelbrotConfig {
        iteration_max: 1348,
        palette: palette_blue_to_white_circle_up(),
        palette_zero: palette_gray_to_black_circle_down(),
        phantom: Default::default(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 720,
        width_re: 3.5,
        center_re: -0.088485445553580480,
        center_im: -0.200679435068532800,
    };
    let application: Application<'static> = application::init(area_config, mandelbrot_config);
    let mut mandelbrot: CollatzConjecture<'static> = CollatzConjecture { app: application };
    let app = window::show(&mandelbrot);
    thread::spawn(move || {
        mandelbrot.calculate_mandelbrot();
        FRACTAL.lock().unwrap().replace(mandelbrot);
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_common::mem_collatz::MemCollatz;
    use rusty_fractals_core::application;
    use crate::CollatzConjecture;

    #[test]
    fn test_math() {
        let collatz = CollatzConjecture { app: application::init_none() };
        let mut mc = MemCollatz { m: Mem { re: 0.0, im: 0.0 }, num: 7 };
        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.re(), 2.0);
        assert_eq!(mc.im(), 1.1);
    }
}