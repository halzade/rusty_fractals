use std::sync::Mutex;
use std::thread;
use rusty_fractals_core::{application, machine_mandelbrot, window};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{FractalApplication, FractalCommon, FractalMandelbrotCommon, FractalMath, MandelbrotConfig};
use rusty_fractals_common::fractal_log::now;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::palettes::{palette_blue_to_white_circle_up, palette_gray_to_black_circle_down};
use rusty_fractals_core::application::Application;

pub struct Mandelbrot<'lt> {
    app: Application<'lt>,
}

impl FractalMath<Mem> for Mandelbrot<'_> {
    fn math(&self, mc: &mut Mem, origin_re: f64, origin_im: f64) {
        mc.square();
        mc.plus(origin_re, origin_im);
    }
}

impl FractalMandelbrotCommon for Mandelbrot<'_> {
    fn calculate_path(&self, iteration_max: u32, origin_re: f64, origin_im: f64) -> (u32, f64) {
        fractal::calculate_mandelbrot_path(self, iteration_max, origin_re, origin_im)
    }
    fn calculate_mandelbrot(&mut self) {
        let fm = machine_mandelbrot::init();
        fm.calculate_mandelbrot(self);
    }
    fn palette_zero(&self) -> &Palette {
        &self.app.palette_zero
    }
}

impl FractalCommon for Mandelbrot<'_> {
    fn name(&self) -> &'static str { "Mandelbrot" }
    fn update(&mut self) { self.app.conf_add(0, 150); }
    fn zoom_in(&mut self) { self.app.zoom_in(); }
    fn recalculate_pixels_positions_for_next_calculation(&self, is_mandelbrot: bool) {
        self.app.recalculate_pixels_positions_for_next_calculation(is_mandelbrot);
    }
    fn move_target(x: usize, y: usize) {
        println!("move_target()");
        FRACTAL.lock().unwrap().as_mut().unwrap().app.move_target(x, y);
    }
    fn zoom_and_recalculate() {
        println!("zoom_and_recalculate()");
        FRACTAL.lock().unwrap().as_mut().unwrap().app.zoom_in_recalculate_pixel_positions(true);
        FRACTAL.lock().unwrap().as_mut().unwrap().calculate_mandelbrot_new_thread(&FRACTAL);
    }
}

pub static FRACTAL: Mutex<Option<Mandelbrot>> = Mutex::new(None);

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
        width_re: 4.5,
        center_re: -0.5,
        center_im: 0.0,
    };
    let application: Application<'static> = application::init(area_config, mandelbrot_config);
    let mandelbrot: Mandelbrot<'static> = Mandelbrot { app: application };
    let app = window::show(mandelbrot);

        let machine = machine_mandelbrot::init();
        machine.calculate_mandelbrot(&mandelbrot);
        // FRACTAL.lock().unwrap().replace(mandelbrot);

    app.run().unwrap();

    now(mandelbrot.name())
}

impl<'lt> FractalApplication for Mandelbrot<'lt> {
    fn width(&self) -> usize { self.app.width }
    fn height(&self) -> usize { self.app.height }
    fn data(&self) -> &DataImage<'static> { & self.app.data }
    fn palette(&self) -> &Palette { &self.app.palette }
    fn min(&self) -> u32 { self.app.conf.min }
    fn max(&self) -> u32 { self.app.conf.max }
    fn area(&self) -> &Area<'_> { &self.app.area }
}
