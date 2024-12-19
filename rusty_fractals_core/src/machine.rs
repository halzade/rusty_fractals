use crate::window;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use crate::area::{Area, AreaConfig};
use crate::calc::CalculationConfig;
use crate::data_image::{state_from_path_length, DataImage};
use crate::fractal::{FractalConfig, FractalMath, MemType};
use crate::perfect_colour_distribution::perfectly_colour_nebula_values;
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::{fractal, pixel_states};

// to calculate single image
pub struct Machine {}

pub fn init() -> Machine {
    Machine {}
}

impl Machine {
    pub fn calculate<'lt, M: MemType<M>>(
        fractal: &dyn FractalMath<M>,
        fractal_config: FractalConfig,
        area_config: AreaConfig,
        calc_config: CalculationConfig,
    ) {
        println!("calculate()");
        let coordinates_xy: Vec<[u32; 2]> = shuffled_calculation_coordinates();
        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation(&xy, fractal);
            // window refresh
            window::paint_image_calculation_progress(fractal.data_image());
        });
        fractal.data_image().recalculate_pixels_states();

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
    fn chunk_calculation<'lt, F: FractalNebulaCommon<'lt> + FractalCommon<'lt>>(
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

    fn chunk_calculation_with_wrap<'lt, F: FractalNebulaCommon<'lt> + FractalCommon<'lt>>(
        &self,
        xy: &[u32; 2],
        fractal: &'static F,
    ) {
        if fractal.rm() == ResolutionMultiplier::Single {
            panic!()
        }
        let (x_from, x_to, y_from, y_to) = chunk_boundaries(xy, fractal.width(), fractal.height());
        let data = fractal.data_image();
        let area = fractal.area();
        let plank = area.plank();
        for x in x_from..x_to {
            for y in y_from..y_to {
                if data.is_on_mandelbrot_horizon(x, y) {
                    let (origin_re, origin_im) = data.origin_at(x, y);
                    let wrap = data.wrap(origin_re, origin_im, fractal.rm(), plank);
                    // within the same pixel
                    for [re, im] in wrap {
                        fractal.calculate_path(
                            area,
                            fractal.min(),
                            fractal.max(),
                            re,
                            im,
                            fractal.data_image(),
                            true,
                        );
                    }
                }
            }
        }
    }

    fn calculate_path_xy<'lt, M: MemType<M>>(
        x: usize,
        y: usize,
        min: u32,
        max: u32,
        fractal: &dyn FractalMath<M>,
        area: Area,
        data_image: DataImage,
        calc_config: CalculationConfig,
    ) {
        let (state, origin_re, origin_im) = data_image.state_origin_at(x, y);
        if pixel_states::is_active_new(state) {
            let (iterator, path_length) = fractal::calculate_path(
                fractal,
                &area,
                min,
                max,
                origin_re,
                origin_im,
                &data_image,
                calc_config,
                false,
            );
            let state = state_from_path_length(iterator, path_length, fractal.min(), fractal.max());
            data_image.set_pixel_state(x, y, state);
        }
    }
}

pub fn chunk_boundaries(
    xy: &[u32; 2],
    width: usize,
    height: usize,
) -> (usize, usize, usize, usize) {
    let chunk_size_x = (width / 20) as u32;
    let chunk_size_y = (height / 20) as u32;
    (
        (xy[0] * chunk_size_x) as usize,
        ((xy[0] + 1) * chunk_size_x) as usize,
        (xy[1] * chunk_size_y) as usize,
        ((xy[1] + 1) * chunk_size_y) as usize,
    )
}

/**
 * Creates x,y pairs for calculation.
 * Then shuffles them, it looks better when rendering
 */
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

#[cfg(test)]
mod tests {
    use crate::machine::Machine;
    use crate::{area, calc, data_image, fractal};

    #[test]
    fn test_calculate_path_xy() {
        let fractal = fractal::init_trivial();
        let area = area::init_trivial();
        let data_image = data_image::init_trivial();
        let calc_config = calc::init_trivial();
        Machine::calculate_path_xy(0, 0, 1, 5, &fractal, area, data_image, calc_config);

        // TODO
    }
}
