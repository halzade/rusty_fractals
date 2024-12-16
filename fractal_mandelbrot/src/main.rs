use std::thread;
use rusty_fractals_core::{application, machine_mandelbrot, window};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{FractalCommon, FractalMandelbrotCommon, FractalMath, MandelbrotConfig};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::palettes::{palette_blue_to_white_circle_up, palette_gray_to_black_circle_down};
use rusty_fractals_core::application::Application;

/**
 * The Mandelbrot Fractal
 */
pub struct Mandelbrot {
    name: &'static str,
}

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

    // TODO
    let application: Application<'static> = application::init(area_config, mandelbrot_config);

    let mandelbrot: &Mandelbrot = &Mandelbrot { name: "Mandelbrot" };
    let app = window::show(mandelbrot);

    thread::spawn(move || {
        let machine = machine_mandelbrot::init();
        machine.calculate_mandelbrot(mandelbrot);


        // TODO don't replace FRACTAl mandelbrot, cycle instead with waiting
    });

    app.run().unwrap();
}

impl FractalMandelbrotCommon for Mandelbrot {
    fn calculate_path(&self, iteration_max: u32, origin_re: f64, origin_im: f64) -> (u32, f64) {
        fractal::calculate_mandelbrot_path(self, iteration_max, origin_re, origin_im)
    }
    fn calculate_mandelbrot(&self) {
        let fm = machine_mandelbrot::init();
        fm.calculate_mandelbrot(self);
    }
    fn palette_zero(&self) -> &Palette {
        &self.app.palette_zero
    }
}

impl FractalCommon for Mandelbrot<> {
    fn name(&self) -> &'static str { "Mandelbrot" }
    fn update(&self) { self.conf_add(0, 150); }
    fn zoom_in(&self) { self.zoom_in(); }
    fn data(&self) -> &DataImage<'static> { &self.data }
    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }
    fn data_image(&self) -> &DataImage<'static> {
        todo!()
    }
    fn palette(&self) -> &Palette { &self.app.palette_zero }
    fn min(&self) -> u32 { self.conf.min }
    fn max(&self) -> u32 { self.conf.max }
    fn area(&self) -> &Area<'_> { &self.area }
    fn recalculate_pixels_positions_for_next_calculation(&self, is_mandelbrot: bool) {
        self.recalculate_pixels_positions_for_next_calculation(is_mandelbrot);
    }
    fn move_target(&self, x: usize, y: usize) {
        println!("move_target()");
        self.move_target(x, y);
    }

    fn zoom_and_recalculate(&self) {
        println!("zoom_and_recalculate()");
        self.zoom_in_recalculate_pixel_positions(true);

        // TODO
        // FRACTAL.unwrap().calculate_mandelbrot_new_thread(FRACTAL);
        self.calculate_mandelbrot();
    }
}
