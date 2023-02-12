use image::{RgbImage};
use rayon::prelude::*;
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Fractal};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::constants::CALCULATION_BOUNDARY;
use rusty_fractals_common::{area, mem, result_data_static};
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
    repeat: bool,
    save_images: bool,
    palette: &'lt Palette,
}

pub fn init<'lt>(calculation_config: &CalculationConfig, app_config: &AppConfig, result_config: &'lt ResultConfig, area_config: &AreaConfig) -> Machine<'lt> {
    let area = area::init(&area_config);
    let domain = domain::init(&area);
    Machine {
        area,
        domain,
        iteration_min: calculation_config.iteration_min,
        iteration_max: calculation_config.iteration_max,
        resolution_multiplier: calculation_config.resolution_multiplier,
        repeat: app_config.repeat,
        save_images: app_config.save_images,
        palette: &result_config.palette,
    }
}

impl Machine<'_> {
    pub fn calculate(&self, fractal: &impl Fractal<Mem>) -> (RgbImage, RgbImage) {
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
        fractal: &impl Fractal<Mem>,
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
        fractal: &impl Fractal<Mem>,
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
                        self.calculate_path(re, im, fractal, result_static);
                    }
                }
            }
        }
    }

    fn calculate_path_xy(
        &self, x: usize, y: usize,
        fractal: &impl Fractal<Mem>,
        result_static: &ResultDataStatic,
    ) {
        let (state, origin_re, origin_im) = self.domain.get_el_triplet(x, y);
        if pixel_states::is_active_new(state) {
            let iterator = self.calculate_path(origin_re, origin_im, fractal, result_static);
            self.domain.set_finished_state(
                x, y,
                Domain::state_from_path_length(iterator, self.iteration_min, self.iteration_max),
            );
        }
    }

    fn calculate_path(
        &self, origin_re: f64, origin_im: f64,
        fractal: &impl Fractal<Mem>,
        result_static: &ResultDataStatic,
    ) -> u32 {
        let cb = CALCULATION_BOUNDARY as f64;

        let mut m = mem::new(origin_re, origin_im);
        let mut iterator = 0;
        let mut length = 0;

        while m.quad() < cb && iterator < self.iteration_max {

            // Investigate if this is a good calculation path
            // Don't create path data yet. Too many origins don't produce good data
            // Most of the long and expensive calculations end up inside Mandelbrot set, useless
            // It is 1.68x faster to calculate path twice, and to record exclusively the good paths

            fractal.math(&mut m, origin_re, origin_im);
            if self.area.contains(m.re, m.im) {
                // TODO comment why I shouldn't remove this
                length += 1;
            }
            iterator += 1;
        }

        if fractal.path_test(self.iteration_min, self.iteration_max, length, iterator) {

            // This origin produced good data
            // Record the calculation path

            let mut m = mem::new(origin_re, origin_im);
            let mut path: Vec<[f64; 2]> = Vec::new();
            for _ in 0..iterator {
                fractal.math(&mut m, origin_re, origin_im);
                if self.area.contains(m.re, m.im) {
                    path.push([m.re, m.im]);
                }
            }
            result_static.translate_path_to_point_grid(path, &self.area);
            // stats.paths_new_points_amount += path.size();
        }
        iterator
    }
}
