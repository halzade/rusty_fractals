use image::{RgbImage};
use rayon::prelude::*;
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal::{CalculationConfig, Fractal};
use rusty_fractals_common::{area, result_data_static};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_common::result_data_static::ResultDataStatic;
use rusty_fractals_domain::{domain, pixel_states};
use rusty_fractals_domain::domain::Domain;
use rusty_fractals_result::palette::Palette;
use rusty_fractals_result::result_pixels;
use rusty_fractals_result::perfect_color_distribution::perfectly_color_result_values;
use rusty_fractals_result::result::ResultConfig;

// to calculate single image
pub struct Machine<'lt> {
    area: Area,
    domain: Domain,
    iteration_min: u32,
    iteration_max: u32,
    resolution_multiplier: ResolutionMultiplier,
    palette: &'lt Palette,
}

pub fn init<'lt>(calculation_config: &CalculationConfig, result_config: &'lt ResultConfig, area_config: &AreaConfig) -> Machine<'lt> {
    let area = area::init(&area_config);
    let domain = domain::init(&area);
    Machine {
        area,
        domain,
        iteration_min: calculation_config.iteration_min,
        iteration_max: calculation_config.iteration_max,
        resolution_multiplier: calculation_config.resolution_multiplier,
        palette: &result_config.palette,
    }
}

impl Machine<'_> {
    pub fn calculate(&self, fractal: &impl Fractal) -> (RgbImage, RgbImage) {
        println!("calculate()");
        let coordinates_xy: Vec<[u32; 2]> = self.domain.shuffled_calculation_coordinates();

        let result_static = result_data_static::init(&self.area);
        coordinates_xy
            .par_iter()
            .for_each(|xy| {
                self.chunk_calculation(&xy, fractal, &result_static);
            });

        self.domain.recalculate_pixels_states(&self.area);

        if self.resolution_multiplier != ResolutionMultiplier::Single {
            println!("calculate() with wrap");
            // previous calculation completed, calculate more elements
            coordinates_xy
                .par_iter()
                .for_each(|xy| {
                    self.chunk_calculation_with_wrap(&xy, fractal, &result_static);
                });
        }
        let mut result_pixels = result_pixels::init(&self.area);
        result_pixels.translate_all_points_to_pixel_grid(result_static.all_points(), &self.area);

        let domain_image = self.domain.domain_element_states_to_image();
        let result_image = perfectly_color_result_values(&result_pixels, &self.palette);
        (domain_image, result_image)
    }

    fn chunk_boundaries(&self, xy: &[u32; 2]) -> (usize, usize, usize, usize) {
        let chunk_size_x = (self.area.width_x / 20) as u32;
        let chunk_size_y = (self.area.height_y / 20) as u32;

        ((xy[0] * chunk_size_x) as usize,
         ((xy[0] + 1) * chunk_size_x) as usize,
         (xy[1] * chunk_size_y) as usize,
         ((xy[1] + 1) * chunk_size_y) as usize)
    }

    // in sequence executes as 20x20 parallel for each domain chunk
    fn chunk_calculation(
        &self, xy: &[u32; 2],
        fractal: &impl Fractal,
        result_static: &ResultDataStatic,
    ) {
        let (x_from, x_to, y_from, y_to) = self.chunk_boundaries(xy);
        for x in x_from..x_to {
            for y in y_from..y_to {
                self.calculate_path_xy(x, y, fractal, result_static);
            }
        }
    }

    fn chunk_calculation_with_wrap(
        &self, xy: &[u32; 2],
        fractal: &impl Fractal,
        result_static: &ResultDataStatic,
    ) {
        if self.resolution_multiplier == ResolutionMultiplier::Single {
            panic!()
        }
        let (x_from, x_to, y_from, y_to) = self.chunk_boundaries(xy);
        for x in x_from..x_to {
            for y in y_from..y_to {
                if self.domain.is_on_mandelbrot_horizon(x, y) {
                    let (_, origin_re, origin_im) = self.domain.get_el_triplet(x, y);
                    let wrap = self.domain.wrap(origin_re, origin_im, self.resolution_multiplier, &self.area);
                    for [re, im] in wrap {
                        fractal.calculate_path(&self.area, self.iteration_min, self.iteration_max, re, im, result_static);
                    }
                }
            }
        }
    }

    fn calculate_path_xy(
        &self, x: usize, y: usize,
        fractal: &impl Fractal,
        result_static: &ResultDataStatic,
    ) {
        let (state, origin_re, origin_im) = self.domain.get_el_triplet(x, y);
        if pixel_states::is_active_new(state) {
            let (iterator, path_length) = fractal.calculate_path(&self.area, self.iteration_min, self.iteration_max, origin_re, origin_im, result_static);

            let state = Domain::state_from_path_length(iterator, path_length, self.iteration_min, self.iteration_max);
            self.domain.set_finished_state(x, y, state);
        }
    }
}
