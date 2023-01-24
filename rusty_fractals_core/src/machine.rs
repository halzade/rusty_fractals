use rusty_fractals_result::{perfect_color_distribution, result_pixels};
use rusty_fractals_common::area::Area;
use rusty_fractals_domain::domain::Domain;
use image::{RgbImage};
use perfect_color_distribution::perfectly_color_result_values;
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Math};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::result_data::ResultData;
use rusty_fractals_result::result::ResultConfig;

// to calculate single image
pub struct Machine<'lif> {
    pub area: &'lif Area,
    pub domain: &'lif mut Domain<'lif>,
    pub calculation_config: CalculationConfig,
    pub app_config: AppConfig,
    pub result_config: ResultConfig,
}

impl Machine<'_> {
    pub fn calculate(&mut self, fractal_math: &impl Math<Mem>) -> (RgbImage, RgbImage) {
        println!("calculate()");
        let coordinates_xy = self.domain.shuffled_calculation_coordinates();

        let mut result_data = ResultData { paths: Vec::new() };

        coordinates_xy
            .iter()
            .for_each(|xy| self.chunk_calculation(&xy, fractal_math, &mut result_data));

        let mut result_pixels = result_pixels::init(self.area.width_x, self.area.height_y);
        result_pixels.translate_paths_to_pixel_grid(result_data.paths, &self.area);

        let domain_image = self.domain.domain_element_states_to_image();
        let result_image = perfectly_color_result_values(&result_pixels, &self.result_config.palette);
        (domain_image, result_image)
    }

    // in sequence (cpu_num) executes as 20x20 parallel for each domain chunk
    pub fn chunk_calculation(&mut self, xy: &[u32; 2], fractal_math: &impl Math<Mem>, result: &mut ResultData) {
        let chunk_size_x = (self.domain.width / 20) as u32;
        let chunk_size_y = (self.domain.height / 20) as u32;

        let x_from = (xy[0] * chunk_size_x) as usize;
        let x_to = ((xy[0] + 1) * chunk_size_x) as usize;
        let y_from = (xy[1] * chunk_size_y) as usize;
        let y_to = ((xy[1] + 1) * chunk_size_y) as usize;
        for x in x_from..x_to {
            for y in y_from..y_to {
                self.domain.calculate_path_finite(x, y, fractal_math, result, &self.calculation_config);
            }
        }
    }
}
