use perfect_color_distribution::perfectly_color_values;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rusty_fractals_result::perfect_color_distribution;
use rusty_fractals_result::result_data::ResultData;
use rusty_fractals_result::result_pixels::ResultPixels;
use rusty_fractals_common::area::Area;
use rusty_fractals_common::constants::CALCULATION_BOUNDARY;
use rusty_fractals_domain::domain::Domain;
use rusty_fractals_domain::domain_element::DomainElement;
use log::{debug};
use rusty_fractals_domain::pixel_states::DomainElementState;
use crate::fractal::{AppConfig, CalculationConfig, Math, ResultConfig};
use crate::mem::Mem;

// to calculate single image
pub struct Machine<'lif> {
    pub area: &'lif Area,
    pub domain: &'lif Domain<'lif>,
    pub calculation_config: CalculationConfig,
    pub app_config: AppConfig,
    pub result_config: ResultConfig,
}

impl Machine<'_> {
    pub fn calculate(&mut self, fractal_math: &impl Math<Mem>) {
        let coordinates_xy = self.domain.shuffled_calculation_coordinates();

        let mut result_data = ResultData {
            paths: Vec::new()
        };

        // Calculate independently and in parallel each domain chunks
        // TODO coordinates_xy.into_par_iter().for_each(
        coordinates_xy.iter().for_each(
            |xy| self.chunk_calculation(&xy, fractal_math, &mut result_data)
        );

        let mut result_pixels = ResultPixels {
            width: self.area.width_x,
            height: self.area.height_y,
            pixels: vec![],
        };

        // TODO self.translate_paths_to_pixel_grid(&result_data, &mut result_pixels);

        // self.domain.mask_full_update(); TODO

        // TODO let result_image = perfectly_color_values(&mut result_pixels, &self.result_config.palette);
        // TOOD Application.repaint_mandelbrot_window();
    }

    // in sequence (cpu_num) executes as 20x20 parallel for each domain chunk
    pub fn chunk_calculation(&self, xy: &[u32; 2], fractal_math: &impl Math<Mem>, result: &mut ResultData) {
        let chunk_size_x = (self.domain.width / 20) as u32;
        let chunk_size_y = (self.domain.height / 20) as u32;

        let x_from = (xy[0] * chunk_size_x) as usize;
        let x_to = ((xy[0] + 1) * chunk_size_x) as usize;
        let y_from = (xy[1] * chunk_size_y) as usize;
        let y_to = ((xy[1] + 1) * chunk_size_y) as usize;
        for x in x_from..x_to {
            for y in y_from..y_to {
                let core_element: &DomainElement = self.domain.domain_elements[x]
                    .get(y)
                    .expect("domain_elements problem");
                if core_element.is_active_new() {
                    self.calculate_path_finite(core_element, fractal_math, result);
                }
            }
        }
    }

    pub fn calculate_path_finite(&self, el: &DomainElement, fractal_math: &impl Math<Mem>, result: &mut ResultData) -> DomainElementState {
        let max = self.calculation_config.iteration_max;
        let min = self.calculation_config.iteration_min;
        let cb = CALCULATION_BOUNDARY as f64;
        let mut iterator = 0;
        let mut length = 0;
        let mut m = Mem::new(el.origin_re, el.origin_im);
        while m.quad() < cb && iterator < max {

            // Investigate if this is a good calculation path
            // Don't create path data yet. Too many origins don't produce good data
            // Most of the long and expensive calculations end up inside Mandelbrot set, useless
            // It is 1.68x faster to calculate path twice, and recording exclusively the good paths

            fractal_math.math(&mut m, el.origin_re, el.origin_im);
            if self.area.contains(m.re, m.im) {
                length += 1;
            }
            iterator += 1;
        }
        let el_state = Domain::state_from_path_length(iterator, max, min);

        if length > min && iterator < max {

            // This origin produced good data, record calculation path

            let mut m = Mem::new(el.origin_re, el.origin_im);
            // TODO el.good_path();

            let mut path: Vec<[f64; 2]> = Vec::new();
            for _ in 0..iterator {
                fractal_math.math(&mut m, el.origin_re, el.origin_im);
                if self.area.contains(m.re, m.im) {
                    path.push([m.re, m.im]);
                }
            }
            result.add_calculation_path(path);
            // stats.paths_new_points_amount += path.size();
        }

        el_state
    }

    pub fn translate_paths_to_pixel_grid(&self, result_data: &ResultData, result_pixels: &mut ResultPixels) {
        /*
        debug!("translate_paths_to_pixel_grid()");

        let mut pixels_total = 0;

        for path in result_data.paths {
            for re_im in &path {
                // translate [re,im] to [px,py]
                let re = re_im[0];
                let im = re_im[1];
                if self.area.contains(re, im) {
                    let (px, py) = self.area.domain_point_to_result_pixel(re, im);
                    result_pixels.add(px, py);
                    pixels_total += 1;
                }
            }
        }
        debug!("pixels_total:   {}", pixels_total);

        // remove elements which moved out of tiny area
        // TODO self.remove_elements_outside();

        // Stats.pathsTotalAmount = PATHS.size();
        // Stats.pixelsValueTotal = pixels_total;
        */
    }
}
