use std::sync::{Arc, Mutex};
use fltk::app;
use fltk::app::{Receiver, Sender};
use rayon::prelude::*;
use rusty_fractals_common::fractal::{FractalMandelbrotCommon, FractalCommon, FractalApplication};
use rusty_fractals_common::data_image::state_from_path_length;
use rusty_fractals_common::perfect_colour_distribution::perfectly_colour_mandelbrot_values;
use rusty_fractals_common::pixel_states;
use crate::machine;
use crate::machine::MachineRefresh;

// to calculate single image
pub struct MachineMandelbrot {
    sender: Sender<Vec<u8>>,
}

pub fn init() -> MachineMandelbrot {
    let (sender, _): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = app::channel();
    MachineMandelbrot {
        sender,
    }
}

impl MachineRefresh for MachineMandelbrot {
    fn sender(&self) -> &Sender<Vec<u8>> {
        &self.sender
    }
}

impl MachineMandelbrot {
    pub fn calculate_mandelbrot<M: FractalMandelbrotCommon + FractalCommon + FractalApplication>(&self, fractal: &M) {
        println!("calculate_mandelbrot()");
        let coordinates_xy: Vec<[u32; 2]> = machine::shuffled_calculation_coordinates();
        let refresh_lock = Arc::new(Mutex::new(true));
        let data = fractal.data();
        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation_mandelbrot(fractal, xy);
            // window refresh
            self.refresh_main(&refresh_lock, data);
        });
        data.recalculate_pixels_states();
        let palette = fractal.palette();
        let palette_zero = fractal.palette_zero();
        perfectly_colour_mandelbrot_values(&data, &palette, &palette_zero);
        self.refresh_final(&data);
    }

    fn chunk_calculation_mandelbrot<M: FractalMandelbrotCommon + FractalCommon + FractalApplication>(
        &self,
        fractal: &M,
        xy: &[u32; 2],
    ) {
        let (x_from, x_to, y_from, y_to) = machine::chunk_boundaries(xy, fractal.width(), fractal.height());
        let data = fractal.data();
        let conf = fractal.conf();
        for x in x_from..x_to {
            for y in y_from..y_to {
                let (state, origin_re, origin_im) = data.state_origin_at(x, y);
                // TODO, calculate only ActiveNew elements, copy quad and quid
                if !pixel_states::is_finished_any(state) {
                    // calculation
                    let (iterator, quad) = fractal.calculate_path(conf.max, origin_re, origin_im);
                    let state = state_from_path_length(iterator, iterator, 0, conf.max);
                    data.set_pixel_mandelbrot(x, y, iterator, quad, state, conf.max);
                }
            }
        }
    }
}