use std::sync::{Arc, Mutex};
use rand::thread_rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use rusty_fractals_common::{area, perfect_color_distribution, pixel_states};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal::{CalculationConfig, Fractal};
use rusty_fractals_common::data_image::{DataImage, state_from_path_length};
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::palettes::{ResultConfig};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;

// to calculate single image
pub struct Machine<'lt> {
    area: Area,
    iteration_min: u32,
    iteration_max: u32,
    resolution_multiplier: ResolutionMultiplier,
    palette: &'lt Palette,
}

pub fn init<'lt>(calculation_config: &CalculationConfig, result_config: &'lt ResultConfig, area_config: &AreaConfig) -> Machine<'lt> {
    let area = area::init(&area_config);
    Machine {
        area,
        iteration_min: calculation_config.iteration_min,
        iteration_max: calculation_config.iteration_max,
        resolution_multiplier: calculation_config.resolution_multiplier,
        palette: &result_config.palette,
    }
}

impl Machine<'_> {
    pub fn calculate(&self, fractal: &impl Fractal, data_image: &DataImage) {
        println!("calculate()");
        let coordinates_xy: Vec<[u32; 2]> = shuffled_calculation_coordinates();

        coordinates_xy.par_iter().for_each(|xy| {
            self.chunk_calculation(&xy, fractal, &data_image);
        });
        data_image.recalculate_pixels_states();

        if self.resolution_multiplier != ResolutionMultiplier::Single {
            println!("calculate() with wrap");
            // previous calculation completed, calculate more elements
            coordinates_xy.par_iter().for_each(|xy| {
                self.chunk_calculation_with_wrap(&xy, fractal, &data_image);
            });
        }
        perfect_color_distribution::perfectly_color_result_values(&data_image, &self.palette);
    }

    // in sequence executes as 20x20 parallel for each image part/chunk
    fn chunk_calculation(
        &self, xy: &[u32; 2],
        fractal: &impl Fractal,
        data_image: &DataImage,
    ) {
        let (x_from, x_to, y_from, y_to) = chunk_boundaries(xy, self.area.width_x, self.area.height_y);
        for x in x_from..x_to {
            for y in y_from..y_to {
                self.calculate_path_xy(x, y, fractal, data_image);
            }
        }
    }

    fn chunk_calculation_with_wrap(
        &self, xy: &[u32; 2],
        fractal: &impl Fractal,
        data_image: &DataImage,
    ) {
        if self.resolution_multiplier == ResolutionMultiplier::Single {
            panic!()
        }
        let (x_from, x_to, y_from, y_to) = chunk_boundaries(xy, self.area.width_x, self.area.height_y);
        for x in x_from..x_to {
            for y in y_from..y_to {
                if data_image.is_on_mandelbrot_horizon(x, y) {
                    let (_, _, origin_re, origin_im) = data_image.values_at(x, y);
                    let wrap = data_image.wrap(origin_re, origin_im, self.resolution_multiplier, &self.area);
                    // within the same pixel
                    for [re, im] in wrap {
                        fractal.calculate_path(&self.area, self.iteration_min, self.iteration_max, re, im, data_image);
                    }
                }
            }
        }
    }

    fn calculate_path_xy(
        &self, x: usize, y: usize,
        fractal: &impl Fractal,
        data_image: &DataImage,
    ) {
        let (_, state, origin_re, origin_im) = data_image.values_at(x, y);
        if pixel_states::is_active_new(state) {
            let (iterator, path_length) = fractal.calculate_path(&self.area, self.iteration_min, self.iteration_max, origin_re, origin_im, data_image);

            let state = state_from_path_length(iterator, path_length, self.iteration_min, self.iteration_max);
            data_image.set_pixel_state(x, y, state);
        }
    }

    pub fn area(&self) -> &Area {
        &self.area
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
