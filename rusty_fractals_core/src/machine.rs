use rand::thread_rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use rusty_fractals_common::{pixel_states};
use rusty_fractals_common::fractal::{FractalNebulaCommon, FractalCommon};
use rusty_fractals_common::data_image::{state_from_path_length};
use rusty_fractals_common::perfect_colour_distribution::perfectly_colour_nebula_values;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use crate::window;

// to calculate single image
pub struct Machine {}

pub fn init() -> Machine {
    Machine {}
}

impl Machine {
    pub fn calculate<F: FractalNebulaCommon + FractalCommon>(&self, fractal: &'static F) {
        println!("calculate()");
        let coordinates_xy: Vec<[u32; 2]> = shuffled_calculation_coordinates();
        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation(&xy, fractal);
            // window refresh
            window::paint_image_calculation_progress(fractal);
        });
        fractal.recalculate_pixels_states();

        let area = fractal.area();
        if fractal.rm() != ResolutionMultiplier::Single {
            println!("calculate() with wrap");
            // previous calculation completed, calculate more elements
            coordinates_xy.par_iter().for_each(|xy| {
                // calculation
                self.chunk_calculation_with_wrap(&xy, fractal);
                // window refresh
                // TODO window::paint_image_calculation_progress(&data);
                window::paint_path(area, fractal.data_image());
            });
        }
        let palette = fractal.palette();
        perfectly_colour_nebula_values(fractal.data_image(), palette);
        window::paint_image_result(fractal.data_image());
    }

    // in sequence executes as 20x20 parallel for each image part/chunk
    fn chunk_calculation<F: FractalNebulaCommon + FractalCommon>(
        &self,
        xy: &[u32; 2],
        fractal: &F,
    ) {
        let (x_from, x_to, y_from, y_to) = chunk_boundaries(xy, fractal.width(), fractal.height());
        for x in x_from..x_to {
            for y in y_from..y_to {
                self.calculate_path_xy(x, y, fractal);
            }
        }
    }

    fn chunk_calculation_with_wrap<F: FractalNebulaCommon + FractalCommon>(
        &self, xy: &[u32; 2],
        fractal: &'static F,
    ) {
        if fractal.rm() == ResolutionMultiplier::Single {
            panic!()
        }
        let (x_from, x_to, y_from, y_to) = chunk_boundaries(xy, fractal.width(), fractal.height());
        let data = fractal.data();
        let area = fractal.area();
        let plank = area.plank();
        for x in x_from..x_to {
            for y in y_from..y_to {
                if data.is_on_mandelbrot_horizon(x, y) {
                    let (origin_re, origin_im) = data.origin_at(x, y);
                    let wrap = data.wrap(origin_re, origin_im, fractal.rm(), plank);
                    // within the same pixel
                    for [re, im] in wrap {
                        fractal.calculate_path(area, fractal.min(), fractal.max(), re, im, &data, true);
                    }
                }
            }
        }
    }

    fn calculate_path_xy<F: FractalNebulaCommon + FractalCommon + FractalApplication>(
        &self,
        x: usize,
        y: usize,
        fractal: &F,
    ) {
        let data = fractal.data();
        let area = fractal.area();
        let (state, origin_re, origin_im) = data.state_origin_at(x, y);
        if pixel_states::is_active_new(state) {
            let (iterator, path_length) = fractal.calculate_path(area, fractal.min(), fractal.max(), origin_re, origin_im, &data, false);
            let state = state_from_path_length(iterator, path_length, fractal.min(), fractal.max());
            data.set_pixel_state(x, y, state);
        }
    }
}

pub fn chunk_boundaries(xy: &[u32; 2], width: usize, height: usize) -> (usize, usize, usize, usize) {
    let chunk_size_x = (width / 20) as u32;
    let chunk_size_y = (height / 20) as u32;
    ((xy[0] * chunk_size_x) as usize, ((xy[0] + 1) * chunk_size_x) as usize,
     (xy[1] * chunk_size_y) as usize, ((xy[1] + 1) * chunk_size_y) as usize)
}

pub fn shuffled_calculation_coordinates() -> Vec<[u32; 2]> {
    let mut coordinates_xy: Vec<[u32; 2]> = Vec::new();
    for x in 0..20 {
        for y in 0..20 {
            coordinates_xy.push([x, y]);
        }
    }
    coordinates_xy.shuffle(&mut thread_rng());
    coordinates_xy
}
