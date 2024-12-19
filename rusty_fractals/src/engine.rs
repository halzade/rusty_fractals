use std::thread;
use crate::{machine, machine_mandelbrot};
use crate::fractal::{FractalMath, MemType};

// to calculate sequence of images for zoom video
pub struct Engine {}

pub fn init() -> Engine {
    Engine {}
}

pub fn calculate_mandelbrot_zoom<'lt, M: MemType<M>>( fractal: &dyn FractalMath<M>) {
    let machine = machine_mandelbrot::init();
    thread::spawn(move || {
        for it in 1.. {
            println!("{}:", it);
            machine.calculate_mandelbrot(fractal);
        };
    });
    // prepare next frame
    fractal.zoom_in();
    fractal.recalculate_pixels_positions_for_next_calculation(true);
    fractal.update();
}

pub fn calculate_nebula_zoom<'lt, M: MemType<M>>( fractal: &dyn FractalMath<M>) {
    let machine = machine::init();
    thread::spawn(move || {
        for it in 1.. {
            println!("{}:", it);
            machine.calculate(fractal);
        };
    });
    // prepare next frame
    fractal.zoom_in();
    fractal.recalculate_pixels_positions_for_next_calculation(false);
    fractal.update();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}

