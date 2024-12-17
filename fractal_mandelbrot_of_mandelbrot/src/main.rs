use std::thread;
use std::sync::Mutex;
use rusty_fractals_core::{application, machine_mandelbrot, window};
use rusty_fractals_core::application::Application;
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal;
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal::{FractalCommon, FractalMandelbrotCommon, FractalMath, MandelbrotConfig};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::palettes::{palette_blue_to_white_circle_up, palette_gray_to_black_circle_down};

pub struct MandelbrotOfMandelbrot<'lt> {
    app: Application<'lt>,
}

impl FractalMath<Mem> for MandelbrotOfMandelbrot<'_> {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        let r = m.re;
        let i = m.im;
        let or = origin_re;
        let oi = origin_im;
        // f(f(z)) : f = z^2 + c
        m.re = r * r * r * r - 6.0 * r * r * i * i + i * i * i * i + 2.0 * r * r * or - 2.0 * i * i * or - 4.0 * r * i * oi + or * or - oi * oi + or - r;
        m.im = 4.0 * r * r * r * i - 4.0 * r * i * i * i + 4.0 * r * i * or + 2.0 * r * r * oi - 2.0 * i * i * oi + 2.0 * or * oi + oi - i;
    }
}

impl FractalMandelbrotCommon for MandelbrotOfMandelbrot<'_> {
    fn calculate_path(&self, iteration_max: u32, origin_re: f64, origin_im: f64) -> (u32, f64) {

        fractal::calculate_mandelbrot_path(self, iteration_max, origin_re, origin_im)

    }
    fn calculate_mandelbrot(&mut self) {
        machine_mandelbrot::init().calculate_mandelbrot(self);
    }
    fn palette_zero(&self) -> &Palette { &self.app.palette_zero }
}

impl FractalCommon for MandelbrotOfMandelbrot<'_> {
    fn name(&self) -> &'static str { "Mandelbrot of Mandelbrot" }
    fn update(&mut self) {
        let c = self.conf_mut();
        c.max += 150;
        println!("iteration_max = {}", c.max);
    }
    fn move_zoom_recalculate(&mut self, x: usize, y: usize) {
        println!("move_zoom_recalculate()");
        self.app.move_target_zoom_in_recalculate_pixel_positions(x, y, true);
        self.calculate_mandelbrot_new_thread(&FRACTAL);
    }
    fn move_target_zoom_in_recalculate(x: usize, y: usize) {
        println!("move_target_zoom_in_recalculate()");
        FRACTAL.lock().unwrap().as_mut().unwrap().move_zoom_recalculate(x, y);
    }
}

pub static FRACTAL: Mutex<Option<MandelbrotOfMandelbrot>> = Mutex::new(None);

fn main() {
    let mandelbrot_config: MandelbrotConfig<'static> = MandelbrotConfig {
        iteration_max: 2500,
        palette: palette_blue_to_white_circle_up(),
        palette_zero: palette_gray_to_black_circle_down(),
        phantom: Default::default(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 720,
        width_re: 3.5,
        center_re: -0.5,
        center_im: 0.0,
    };
    let application: Application<'static> = application::init(area_config, mandelbrot_config);
    let mut mandelbrot_mandelbrot: MandelbrotOfMandelbrot<'static> = MandelbrotOfMandelbrot { app: application };
    let app = window::show(&mandelbrot_mandelbrot);
    thread::spawn(move || {
        mandelbrot_mandelbrot.calculate_mandelbrot();
        FRACTAL.lock().unwrap().replace(mandelbrot_mandelbrot);
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}

