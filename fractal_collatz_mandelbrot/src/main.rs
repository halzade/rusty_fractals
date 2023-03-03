use std::sync::Mutex;
use std::thread;
use rusty_fractals_core::{application, machine_mandelbrot, window};
use rusty_fractals_common::area::AreaConfig;
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{Conf, FractalCommon, FractalMandelbrotCommon, FractalMath, MandelbrotConfig};
use rusty_fractals_common::mem_collatz::MemCollatz;
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::palettes::{palette_blue_to_white_circle_up, palette_gray_to_blue};
use rusty_fractals_core::application::Application;

pub struct CollatzConjectureMandelbrot<'lt> {
    app: Application<'lt>,
}

impl FractalMath<MemCollatz> for CollatzConjectureMandelbrot<'_> {
    fn math(&self, mc: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        mc.square();
        mc.plus_collatz(origin_re, origin_im);
    }
}

impl FractalMandelbrotCommon for CollatzConjectureMandelbrot<'_> {
    fn calculate_path(&self, iteration_max: u32, origin_re: f64, origin_im: f64) -> (u32, f64) {
        fractal::calculate_mandelbrot_path(self, iteration_max, origin_re, origin_im)
    }
    fn update(&mut self) {
        let c = self.conf_mut();
        c.max += 150;
        println!("iteration_max = {}", c.max);
    }
    fn palette_zero(&self) -> &Palette {
        &self.app.palette_zero
    }
    fn calculate_mandelbrot(&mut self) {
        let fm = machine_mandelbrot::init();
        fm.calculate_mandelbrot(self);
    }
}

impl FractalCommon for CollatzConjectureMandelbrot<'_> {
    fn name(&self) -> &'static str { "Collatz Conjecture Mandelbrot" }
    fn width(&self) -> usize { self.app.width }
    fn height(&self) -> usize { self.app.height }
    fn data(&self) -> &DataImage { &self.app.data }
    fn palette(&self) -> &Palette { &self.app.palette }
    fn max(&self) -> u32 { self.app.conf.max }
    fn conf(&self) -> &Conf { &self.app.conf }
    fn conf_mut(&mut self) -> &mut Conf { &mut self.app.conf }
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

pub static FRACTAL: Mutex<Option<CollatzConjectureMandelbrot>> = Mutex::new(None);

fn main() {
    let mandelbrot_config: MandelbrotConfig<'static> = MandelbrotConfig {
        iteration_max: 14800,
        palette: palette_blue_to_white_circle_up(),
        palette_zero: palette_gray_to_blue(),
        phantom: Default::default(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 720,
        width_re: 3.0,
        center_re: -0.882952991714172300,
        center_im: -0.214699221335319460,
    };
    let application: Application<'static> = application::init(area_config, mandelbrot_config);
    let mut mandelbrot: CollatzConjectureMandelbrot<'static> = CollatzConjectureMandelbrot { app: application };
    let app = window::show(&mandelbrot);
    thread::spawn(move || {
        mandelbrot.calculate_mandelbrot();
        FRACTAL.lock().unwrap().replace(mandelbrot);
    });
    app.run().unwrap();
}


#[cfg(test)]
mod tests {
    use rusty_fractals_core::application;
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_common::mem_collatz::MemCollatz;
    use crate::CollatzConjectureMandelbrot;

    #[test]
    fn test_math() {
        let collatz = CollatzConjectureMandelbrot { app: application::init_none() };
        let mut mc = MemCollatz { m: Mem { re: 0.0, im: 0.0 }, num: 0 };
        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.re(), 2.0);
        assert_eq!(mc.im(), 0.65);
    }
}
