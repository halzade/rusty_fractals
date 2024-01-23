use std::thread;
use rusty_fractals_common::fractal::{FractalApplication, FractalCommon, FractalMandelbrotCommon, FractalNebulaCommon};
use crate::{machine, machine_mandelbrot};

// to calculate sequence of images for zoom video
pub struct Engine {}

pub fn init() -> Engine {
    Engine {}
}

pub fn calculate_mandelbrot_zoom<F: FractalMandelbrotCommon + FractalCommon + FractalApplication + Sync>(fractal: &'static F) {
    let machine = machine_mandelbrot::init();
    thread::spawn(move || {
        for it in 1.. {
            println!("{}:", it);
            machine.calculate_mandelbrot(fractal);
            // prepare next frame
            fractal.zoom_in();
            fractal.recalculate_pixels_positions_for_next_calculation(true);
            fractal.update();
        };
    });
}

pub fn calculate_nebula_zoom<F: FractalNebulaCommon + FractalCommon + FractalApplication>(fractal: &'static F) {
    let machine = machine::init();
    thread::spawn(move || {
        for it in 1.. {
            println!("{}:", it);
            machine.calculate(fractal);
            // prepare next frame
            fractal.zoom_in();
            fractal.recalculate_pixels_positions_for_next_calculation(false);
            fractal.update();
        };
    });
}
