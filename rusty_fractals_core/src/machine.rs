use rusty_fractals_result::{perfect_color_distribution, result_pixels};
use rusty_fractals_common::area::Area;
use rusty_fractals_domain::domain::Domain;
use image::{RgbImage};
use perfect_color_distribution::perfectly_color_result_values;
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Math};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::result_data::ResultData;
use rusty_fractals_result::result::ResultConfig;
use rayon::prelude::*;
use rusty_fractals_common::constants::CALCULATION_BOUNDARY;
use rusty_fractals_common::{mem, result_data};
use rusty_fractals_domain::pixel_states;
use rusty_fractals_domain::pixel_states::DomainElementState::GoodPath;

// to calculate single image
pub struct Machine {
    pub calculation_config: CalculationConfig,
    pub app_config: AppConfig,
    pub result_config: ResultConfig,
}

impl Machine {
    pub fn calculate(&self, fractal_math: &impl Math<Mem>, domain: &Domain, area: &Area) -> (RgbImage, RgbImage) {
        println!("calculate()");
        let coordinates_xy: Vec<[u32; 2]> = domain.shuffled_calculation_coordinates();

        let result_data = result_data::init();
        coordinates_xy
            .par_iter()
            .for_each(|xy| {
                self.chunk_calculation(&xy, fractal_math, domain, area, &result_data);
            });

        domain.recalculate_pixels_states(area);

        println!("calculate() with wrap");
        // previous calculation completed, calculate wrapping
        coordinates_xy
            .par_iter()
            .for_each(|xy| {
                self.chunk_calculation_with_wrap(&xy, fractal_math, domain, area, &result_data);
            });

        let mut result_pixels = result_pixels::init(area.width_x, area.height_y);
        result_pixels.translate_paths_to_pixel_grid(result_data.all_paths(), area);

        let domain_image = domain.domain_element_states_to_image();
        let result_image = perfectly_color_result_values(&result_pixels, &self.result_config.palette);
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

    // in sequence (cpu_num) executes as 20x20 parallel for each domain chunk
    pub fn chunk_calculation(
        &self,
        xy: &[u32; 2],
        fractal_math: &impl Math<Mem>,
        domain: &Domain,
        area: &Area,
        result: &ResultData,
    ) {
        let (x_from, x_to, y_from, y_to) = Machine::chunk_boundaries(xy, domain);
        for x in x_from..x_to {
            for y in y_from..y_to {
                self.calculate_path_finite(x, y, fractal_math, &self.calculation_config, domain, area, result);
            }
        }
    }

    pub fn chunk_calculation_with_wrap(
        &self,
        xy: &[u32; 2],
        fractal_math: &impl Math<Mem>,
        domain: &Domain,
        area: &Area,
        result: &ResultData,
    ) {
        let (x_from, x_to, y_from, y_to) = Machine::chunk_boundaries(xy, domain);
        for x in x_from..x_to {
            for y in y_from..y_to {
                if domain.is_on_mandelbrot_horizon(x, y) {
                    let (_, origin_re, origin_im) = domain.get_el_triplet(x, y);
                    let wrap = domain.wrap(origin_re, origin_im, self.calculation_config.resolution_multiplier, area);
                    for [re, im] in wrap {
                        self.calculate_path_finite_f64(re, im, fractal_math, &self.calculation_config, area, result);
                    }
                }
            }
        }
    }

    fn calculate_path_finite_f64(
        &self,
        re: f64,
        im: f64,
        fractal_math: &impl Math<Mem>,
        calculation_config: &CalculationConfig,
        area: &Area,
        result: &ResultData,
    ) {
        let max = calculation_config.iteration_max;
        let min = calculation_config.iteration_min;
        let cb = CALCULATION_BOUNDARY as f64;
        let mut iterator = 0;
        let mut length = 0;
        let mut m = mem::new(re, im);
        while m.quad() < cb && iterator < max {
            fractal_math.math(&mut m, re, im);
            if area.contains(m.re, m.im) {
                length += 1;
            }
            iterator += 1;
        }

        if length > min && iterator < max {
            let mut m = mem::new(re, im);
            let mut path: Vec<[f64; 2]> = Vec::new();
            for _ in 0..iterator {
                fractal_math.math(&mut m, re, im);
                if area.contains(m.re, m.im) {
                    path.push([m.re, m.im]);
                }
            }
            result.add_calculation_path(path);
            // stats.paths_new_points_amount += path.size(); ?
        }
    }

    fn calculate_path_finite(
        &self,
        x: usize,
        y: usize,
        fractal_math: &impl Math<Mem>,
        calculation_config: &CalculationConfig,
        domain: &Domain,
        area: &Area,
        result: &ResultData,
    ) {
        let (state, origin_re, origin_im) = domain.get_el_triplet(x, y);
        if pixel_states::is_active_new(state) {
            let max = calculation_config.iteration_max;
            let min = calculation_config.iteration_min;
            let cb = CALCULATION_BOUNDARY as f64;
            let mut iterator = 0;
            let mut length = 0;
            let mut m = mem::new(origin_re, origin_im);
            while m.quad() < cb && iterator < max {

                // Investigate if this is a good calculation path
                // Don't create path data yet. Too many origins don't produce good data
                // Most of the long and expensive calculations end up inside Mandelbrot set, useless
                // It is 1.68x faster to calculate path twice, and to record exclusively the good paths

                fractal_math.math(&mut m, origin_re, origin_im);
                if area.contains(m.re, m.im) {
                    length += 1;
                }
                iterator += 1;
            }
            let el_state = Domain::state_from_path_length(iterator, max, min);

            if length > min && iterator < max {

                // This origin produced good data
                // Record the calculation path
                domain.set_finished_state(x, y, GoodPath);

                let mut m = mem::new(origin_re, origin_im);

                let mut path: Vec<[f64; 2]> = Vec::new();
                for _ in 0..iterator {
                    fractal_math.math(&mut m, origin_re, origin_im);
                    if area.contains(m.re, m.im) {
                        path.push([m.re, m.im]);
                    }
                }
                result.add_calculation_path(path);
                // stats.paths_new_points_amount += path.size();
            }

            domain.set_finished_state(x, y, el_state);
        }
    }
}
