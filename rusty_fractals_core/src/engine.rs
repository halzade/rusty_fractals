use std::thread;
use rusty_fractals_common::fractal::{FractalCommon, FractalMandelbrotCommon, FractalNebulaCommon};
use crate::{machine, machine_mandelbrot};

// to calculate sequence of images for zoom video
pub struct Engine {}

pub fn init() -> Engine {
    Engine {}
}

pub fn calculate_mandelbrot_zoom<F: FractalMandelbrotCommon + FractalCommon + Sync>(fractal: &'static F, fractal_mu: &mut F) {
    let machine = machine_mandelbrot::init();
    thread::spawn(move || {
        for it in 1.. {
            println!("{}:", it);
            machine.calculate_mandelbrot(fractal);
        };
    });
    // prepare next frame
    fractal_mu.zoom_in();
    fractal.recalculate_pixels_positions_for_next_calculation(true);
    fractal_mu.update();
}

pub fn calculate_nebula_zoom<F: FractalNebulaCommon + FractalCommon>(fractal: &'static F, fractal_mu: &mut F) {
    let machine = machine::init();
    thread::spawn(move || {
        for it in 1.. {
            println!("{}:", it);
            machine.calculate(fractal);
        };
    });
    // prepare next frame
    fractal_mu.zoom_in();
    fractal.recalculate_pixels_positions_for_next_calculation(false);
    fractal_mu.update();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}

