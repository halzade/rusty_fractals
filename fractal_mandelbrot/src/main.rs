use rusty_fractals_core::machine_mandelbrot;
use rusty_fractals_common::area::AreaConfig;
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{Conf, FractalMandelbrot, FractalMath, MandelbrotConfig, UpdateMandelbrot};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::palettes::{palette_blue_to_white_circle_up, palette_gray_to_blue};

struct Mandelbrot {
    name: &'static str,
}

impl FractalMath<Mem> for Mandelbrot {
    fn math(&self, mc: &mut Mem, origin_re: f64, origin_im: f64) {
        mc.square();
        mc.plus(origin_re, origin_im);
    }
}

impl FractalMandelbrot for Mandelbrot {
    fn calculate_mandelbrot_path(&self, iteration_max: u32, origin_re: f64, origin_im: f64) -> (u32, f64) {
        fractal::calculate_mandelbrot_path(self, iteration_max, origin_re, origin_im)
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

impl UpdateMandelbrot for Mandelbrot {
    fn update(&self, conf: &mut Conf) {
        conf.max += 150;
        println!("iteration_max = {}", conf.max);
    }
}

fn main() {
    let mandelbrot_config = MandelbrotConfig {
        iteration_max: 2500,
        palette: palette_blue_to_white_circle_up(),
        palette_zero: palette_gray_to_blue(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 720,
        width_re: 4.5,
        center_re: -0.5,
        center_im: 0.0,
    };
    let mandelbrot = &Mandelbrot { name: "Mandelbrot" };
    machine_mandelbrot::mandelbrot_calculation_for(mandelbrot, mandelbrot_config, area_config);
}