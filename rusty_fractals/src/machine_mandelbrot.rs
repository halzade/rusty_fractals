use crate::{machine, window};
use rayon::prelude::*;
use crate::data_image::state_from_path_length;
use crate::fractal::{FractalConfig, FractalMath, MemType};
use crate::perfect_colour_distribution::perfectly_colour_mandelbrot_values;
use crate::pixel_states;

// to calculate single image
pub struct MachineMandelbrot {}

pub fn init() -> MachineMandelbrot {
    MachineMandelbrot {}
}

impl MachineMandelbrot {
    pub fn calculate_mandelbrot<'lt, M: MemType<M>>(
        &self,
        fractal: &dyn FractalMath<M>,
    ) {
        println!("calculate_mandelbrot()");
        let coordinates_xy: Vec<[u32; 2]> = machine::shuffled_calculation_coordinates();

        let data = fractal.data_image();

        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation_mandelbrot(fractal, xy);
            // window refresh
            window::paint_image_calculation_progress(fractal.data_image());
        });
        data.recalculate_pixels_states();
        let palette = fractal.palette();
        let palette_zero = fractal.palette_zero();
        perfectly_colour_mandelbrot_values(&data, &palette, &palette_zero);
        window::paint_image_result(&data);
    }

    fn chunk_calculation_mandelbrot<'lt, M: MemType<M>>(
        &self,
        fractal: &dyn FractalMath<M>,
        xy: &[u32; 2],
    ) {
        let (x_from, x_to, y_from, y_to) =
            machine::chunk_boundaries(xy, fractal.width(), fractal.height());
        let data = fractal.data_image();
        for x in x_from..x_to {
            for y in y_from..y_to {
                let (state, origin_re, origin_im) = data.state_origin_at(x, y);
                // TODO, calculate only ActiveNew elements, copy quad and quid
                if !pixel_states::is_finished_any(state) {
                    // calculation
                    let (iterator, quad) =
                        fractal.calculate_path(fractal.max(), origin_re, origin_im);
                    let state = state_from_path_length(iterator, iterator, 0, fractal.max());
                    data.set_pixel_mandelbrot(x, y, iterator, quad, state, fractal.max());
                }
            }
        }
    }
}
