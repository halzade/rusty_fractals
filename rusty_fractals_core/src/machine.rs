use image::{RgbImage};
use rayon::prelude::*;
use rusty_fractals_common::area::Area;
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Fractal};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::constants::CALCULATION_BOUNDARY;
use rusty_fractals_common::{mem, result_data_static};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_common::result_data_static::ResultDataStatic;
use rusty_fractals_domain::pixel_states;
use rusty_fractals_domain::domain::Domain;
use rusty_fractals_domain::pixel_states::DomainElementState::GoodPath;
use rusty_fractals_result::palette::Palette;
use rusty_fractals_result::result_pixels;
use rusty_fractals_result::perfect_color_distribution::perfectly_color_result_values;
use rusty_fractals_result::result::ResultConfig;

// to calculate single image
pub struct Machine<'lt> {
    pub iteration_min: u32,
    pub iteration_max: u32,
    pub resolution_multiplier: ResolutionMultiplier,
    pub repeat: bool,
    pub save_images: bool,
    pub palette: &'lt Palette,
}

pub fn init<'lt>(calculation_config: &CalculationConfig, app_config: &AppConfig, result_config: &'lt ResultConfig) -> Machine<'lt> {
    Machine {
        iteration_min: calculation_config.iteration_min,
        iteration_max: calculation_config.iteration_max,
        resolution_multiplier: calculation_config.resolution_multiplier,
        repeat: app_config.repeat,
        save_images: app_config.save_images,
        palette: &result_config.palette,
    }
}

impl Machine<'_> {
    pub fn calculate(&self, fractal: &impl Fractal<Mem>, domain: &Domain, area: &Area) -> (RgbImage, RgbImage) {
        println!("calculate()");
        let coordinates_xy: Vec<[u32; 2]> = domain.shuffled_calculation_coordinates();

        let result_static = result_data_static::init(area);
        coordinates_xy
            .par_iter()
            .for_each(|xy| {
                self.chunk_calculation(&xy, fractal, domain, area, &result_static);
            });

        domain.recalculate_pixels_states(area);

        if self.resolution_multiplier != ResolutionMultiplier::None {
            println!("calculate() with wrap");
            // previous calculation completed, calculate more elements
            coordinates_xy
                .par_iter()
                .for_each(|xy| {
                    self.chunk_calculation_with_wrap(&xy, fractal, domain, area, &result_static);
                });
        }
        let mut result_pixels = result_pixels::init(area);
        result_pixels.translate_all_points_to_pixel_grid(result_static.all_points(), area);

        let domain_image = domain.domain_element_states_to_image();
        let result_image = perfectly_color_result_values(&result_pixels, &self.palette);
        (domain_image, result_image)
    }

    fn chunk_boundaries(xy: &[u32; 2], domain: &Domain) -> (usize, usize, usize, usize) {
        let chunk_size_x = (domain.width / 20) as u32;
        let chunk_size_y = (domain.height / 20) as u32;

        ((xy[0] * chunk_size_x) as usize,
         ((xy[0] + 1) * chunk_size_x) as usize,
         (xy[1] * chunk_size_y) as usize,
         ((xy[1] + 1) * chunk_size_y) as usize)
    }

    // in sequence executes as 20x20 parallel for each domain chunk
    fn chunk_calculation(
        &self,
        xy: &[u32; 2],
        fractal: &impl Fractal<Mem>,
        domain: &Domain,
        area: &Area,
        result_static: &ResultDataStatic,
    ) {
        let (x_from, x_to, y_from, y_to) = Machine::chunk_boundaries(xy, domain);
        for x in x_from..x_to {
            for y in y_from..y_to {
                self.calculate_path_finite(x, y, fractal, domain, area, result_static);
            }
        }
    }

    fn chunk_calculation_with_wrap(
        &self,
        xy: &[u32; 2],
        fractal: &impl Fractal<Mem>,
        domain: &Domain,
        area: &Area,
        result_static: &ResultDataStatic,
    ) {
        if self.resolution_multiplier == ResolutionMultiplier::None {
            panic!()
        }
        let (x_from, x_to, y_from, y_to) = Machine::chunk_boundaries(xy, domain);
        for x in x_from..x_to {
            for y in y_from..y_to {
                if domain.is_on_mandelbrot_horizon(x, y) {
                    let (_, origin_re, origin_im) = domain.get_el_triplet(x, y);
                    let wrap = domain.wrap(origin_re, origin_im, self.resolution_multiplier, area);
                    for [re, im] in wrap {
                        self.calculate_path_finite_f64(re, im, fractal, area, result_static);
                    }
                }
            }
        }
    }

    fn calculate_path_finite_f64(
        &self,
        re: f64,
        im: f64,
        fractal: &impl Fractal<Mem>,
        area: &Area,
        result_static: &ResultDataStatic,
    ) {
        let mut m = mem::new(re, im);

        let max = self.iteration_max;
        let min = self.iteration_min;
        let cb = CALCULATION_BOUNDARY as f64;
        let mut iterator = 0;
        let mut length = 0;
        while m.quad() < cb && iterator < max {
            fractal.math(&mut m, re, im);
            if area.contains(m.re, m.im) {
                length += 1;
            }
            iterator += 1;
        }

        if fractal.path_test(min, max, length, iterator) {
            let mut m = mem::new(re, im);
            let mut path: Vec<[f64; 2]> = Vec::new();
            for _ in 0..iterator {
                fractal.math(&mut m, re, im);
                if area.contains(m.re, m.im) {
                    path.push([m.re, m.im]);
                }
            }
            result_static.translate_path_to_point_grid(path, area);
            // stats.paths_new_points_amount += path.size(); ?
        }
    }

    fn calculate_path_finite(
        &self,
        x: usize,
        y: usize,
        fractal: &impl Fractal<Mem>,
        domain: &Domain,
        area: &Area,
        result_static: &ResultDataStatic,
    ) {
        let (state, origin_re, origin_im) = domain.get_el_triplet(x, y);
        if pixel_states::is_active_new(state) {
            let mut m = mem::new(origin_re, origin_im);

            let max = self.iteration_max;
            let min = self.iteration_min;
            let cb = CALCULATION_BOUNDARY as f64;
            let mut iterator = 0;
            let mut length = 0;
            while m.quad() < cb && iterator < max {

                // Investigate if this is a good calculation path
                // Don't create path data yet. Too many origins don't produce good data
                // Most of the long and expensive calculations end up inside Mandelbrot set, useless
                // It is 1.68x faster to calculate path twice, and to record exclusively the good paths

                fractal.math(&mut m, origin_re, origin_im);
                if area.contains(m.re, m.im) {
                    length += 1;
                }
                iterator += 1;
            }
            let el_state = Domain::state_from_path_length(iterator, max, min);

            if fractal.path_test(min, max, length, iterator) {

                // This origin produced good data
                // Record the calculation path
                domain.set_finished_state(x, y, GoodPath);

                let mut m = mem::new(origin_re, origin_im);

                let mut path: Vec<[f64; 2]> = Vec::new();
                for _ in 0..iterator {
                    fractal.math(&mut m, origin_re, origin_im);
                    if area.contains(m.re, m.im) {
                        path.push([m.re, m.im]);
                    }
                }
                result_static.translate_path_to_point_grid(path, area);
                // stats.paths_new_points_amount += path.size();
            }

            domain.set_finished_state(x, y, el_state);
        }
    }
}
