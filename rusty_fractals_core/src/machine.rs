use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rusty_fractals_domain::domain;
use rusty_fractals_domain::domain::Domain;
use rusty_fractals_domain::domain_element::DomainElement;
use crate::{fractal, fractal_path};
use crate::fractal::{CALCULATION_BOUNDARY, CalculationConfig};
use crate::mem::Mem;

// to calculate single image
pub struct Machine {
    pub domain: Domain,
    pub calculation_config: CalculationConfig,
    result: rusty_fractals_result::fractal_result::ResultData,
}

impl Machine {
    pub fn calculate(&self) {
        let coordinates_xy = self.domain.shuffled_calculation_coordinates();

        // Calculate independently and in parallel each domain chunks
        coordinates_xy.into_par_iter().for_each(
            |xy| chunk_calculation(xy)
        );

        PathsFinebrot.translate_paths_to_pixel_grid();
        MaskMandelbrot.mask_full_update();

        fractal.perfectly_color_values();
        Application.repaint_mandelbrot_window();
    }

    pub fn chunk_calculation(&self, xy: [u32; 2]) {
        let chunk_size_x = self.domain.width / 20;
        let chunk_size_y = self.domain.height / 20;

        let wrapped_chunk = self.domain.make_chunk(
            (xy[0] * chunk_size_x) as usize, ((xy[0] + 1) * chunk_size_x) as usize,
            (xy[1] * chunk_size_y) as usize, ((xy[1] + 1) * chunk_size_y) as usize,
        );
        for el in wrapped_chunk {
            calculate_path_finite(&el);
        }
    }

    pub fn calculate_path_finite(&mut self, el: &DomainElement) {
        let max = self.calculation_config.iteration_max;
        let min = self.calculation_config.iteration_min;
        let mut iterator = 0;
        let mut length = 0;
        let m = Mem::new(el.origin_re, el.origin_im);
        while m.quadrance() < CALCULATION_BOUNDARY && iterator < max {

            // Investigate if this is a good calculation path
            // Don't create path data yet. Too many origin's don't produce good data
            // Most of the long and expensive calculations end up inside Mandelbrot set, useless
            // It is 1.68x faster to calculate path twice, and recording exclusively the good paths

            math(m, el.originRe, el.originIm);
            if AreaFinebrot.contains(m) {
                length += 1;
            }
            iterator += 1;
        }
        el.setFinishedState(iterator, length);

        if length > min && iterator < max {
            // This origin produced good data, record calculation path

            m.reset(el.originRe, el.originIm);
            el.goodPath();

            let paath: Vec<[f64; 2]> = Vec::new();

            for i in 0..iterator {
                math(m, el.originRe, el.originIm);
                if AreaFinebrot.contains(m) {
                    path.push([m.re, m.im]);
                }
            }

            self.result.add_escape_path_long(path);
        }
    }
}
