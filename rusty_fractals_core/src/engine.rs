use std::thread;
use rusty_fractals_common::{area, data_image, fractal_stats};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal::{FractalApplication, FractalCommon, FractalConfig, FractalMandelbrotCommon, FractalNebulaCommon, MandelbrotConfig};
use rusty_fractals_common::perfect_colour_distribution::{perfectly_colour_nebula_values};
use crate::{application, machine, machine_mandelbrot, window};
use crate::application::Application;
use crate::machine::MachineRefresh;

// to calculate sequence of images for zoom video
pub struct Engine {}

pub fn init() -> Engine {
    Engine {}
}

pub fn calculate_mandelbrot_zoom<F: FractalMandelbrotCommon + FractalCommon + FractalApplication>(fractal: &'static F) {
    let mut machine = machine_mandelbrot::init();
    let mut area = fractal.area();
    let mut data = fractal.data();
    thread::spawn(move || {
        for it in 1.. {
            println!("{}:", it);
            machine.calculate_mandelbrot(fractal);
            // prepare next frame
            // TODO
            // area.zoom_in();
            // data.recalculate_pixels_positions_for_next_calculation(area, true);
            // fractal.update();
        };
    });
}

pub fn calculate_nebula_zoom<F: FractalNebulaCommon + FractalCommon + FractalApplication>(fractal: &'static F) {
    let mut machine = machine::init();
    let mut area = fractal.area();
    let mut data = fractal.data();
    thread::spawn(move || {
        for it in 1.. {
            println!("{}:", it);
            machine.calculate(fractal);
            // prepare next frame
            // TODO
            // area.zoom_in();
            // data.recalculate_pixels_positions_for_next_calculation(area, false);
            // fractal.update();
        };
    });
}
