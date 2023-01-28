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
use rusty_fractals_common::result_data;

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

        let mut result_pixels = result_pixels::init(area.width_x, area.height_y);
        result_pixels.translate_paths_to_pixel_grid(result_data.all_paths(), area);

        let domain_image = domain.domain_element_states_to_image();
        let result_image = perfectly_color_result_values(&result_pixels, &self.result_config.palette);
        (domain_image, result_image)
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
        let chunk_size_x = (domain.width / 20) as u32;
        let chunk_size_y = (domain.height / 20) as u32;

        let x_from = (xy[0] * chunk_size_x) as usize;
        let x_to = ((xy[0] + 1) * chunk_size_x) as usize;
        let y_from = (xy[1] * chunk_size_y) as usize;
        let y_to = ((xy[1] + 1) * chunk_size_y) as usize;
        for x in x_from..x_to {
            for y in y_from..y_to {
                domain.calculate_path_finite(x, y, fractal_math, &self.calculation_config, area, result);
            }
        }
    }
}
