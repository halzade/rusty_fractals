use std::thread;
use rusty_fractals::{application, machine_mandelbrot, window};
use rusty_fractals::area::{AreaConfig};
use rusty_fractals::calc::CalculationConfig;
use rusty_fractals::calc::CalculationType::InfiniteVideoZoom;
use rusty_fractals::calc::OrbitType::Finite;
use rusty_fractals::fractal::{FractalMath, MandelbrotConfig};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::{palette_blue_to_white_circle_up, palette_gray_to_black_circle_down};
use rusty_fractals::application::Application;

/**
 * The Mandelbrot Fractal
 */
pub struct Mandelbrot {
}

/**
 * x := x^2 + y^2 + x0
 * y := 2xy + y0
 */
impl FractalMath<Mem> for Mandelbrot  {
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
    let calculation_config = CalculationConfig {
        calc_type: InfiniteVideoZoom,
        orbits : Finite,
        update_max : 150,
        update_min : 0,
    };
    // TODO
    let application: Application<'static> = application::init(area_config, mandelbrot_config);

    let mandelbrot: Mandelbrot = Mandelbrot {};
    let app = window::show(&mandelbrot);

    thread::spawn(move || {
        let machine = machine_mandelbrot::init();
        machine.calculate_mandelbrot(&mandelbrot);


        // TODO don't replace FRACTAl mandelbrot, cycle instead with waiting
    });

    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
