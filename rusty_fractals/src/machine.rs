use crate::area::{Area, AreaConfig};
use crate::calc::CalculationConfig;
use crate::data_image::DataType::Static;
use crate::data_image::{state_from_path_length, DataImage};
use crate::data_px::{active_new, hibernated_deep_black};
use crate::fractal::{FractalConfig, FractalMath, MandelbrotConfig, MemType};
use crate::fractal_log::now;
use crate::palette::Palette;
use crate::perfect_colour_distribution::perfectly_colour_nebula_values;
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::{area, data_image, fractal, palettes, pixel_states, window};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;

// to calculate single image
pub struct Machine<'lt> {
    pub fractal_name: &'lt str,
    pub area: Area<'lt>,
    pub data_image: DataImage<'lt>,
    pub width: i32,
    pub height: i32,
    pub min: u32,
    pub max: u32,
    pub palette: Palette<'lt>,
    //  mandelbrot specific
    pub palette_zero: Palette<'lt>,
    //  nebula specific
    pub resolution_multiplier: ResolutionMultiplier,
}

pub fn init<'lt>(name: &'lt str, fractal_config: FractalConfig, area_config: &AreaConfig, calculation_config: CalculationConfig) -> Machine<'lt> {

}

pub fn init_mandelbrot<'lt>(name: &'lt str, fractal_config: FractalConfig, area_config: &AreaConfig, calculation_config: CalculationConfig) -> Machine<'lt> {
    let area: Area<'_> = area::init(area_config);
    let wx = area.data.lock().unwrap().width_x as i32;
    let hy = area.data.lock().unwrap().height_y as i32;
    Machine {
        fractal_name: "",
        data_image: data_image::init(Static, &area),
        width: wx,
        height: hy,
        area,
        min: 0, // mandelbrot fractals calucalte from 0
        max: calculation_config.iteration_max,
        palette: config.palette,
        palette_zero: config.palette_zero,
        resolution_multiplier: ResolutionMultiplier::Single,
    }
}

pub fn init_nebula<'lt>(name: &'lt str, fractal_config: FractalConfig, area_config: &AreaConfig, calculation_config: CalculationConfig) -> Machine<'lt> {
    let area = area::init(area_config);
    let wx = area.data.lock().unwrap().width_x;
    let hy = area.data.lock().unwrap().height_y;
    Machine {
        data_image: data_image::init(Static, &area),
        width: wx,
        height: hy,
        area,
        min: 0,
        max: config.iteration_max,
        palette: config.palette,
        palette_zero: palettes::init_trivial(),
        resolution_multiplier: config.resolution_multiplier,
    }
}

pub fn init_trivial<'lt>() -> Machine<'lt> {
    Machine {
        data_image: data_image::init_trivial(),
        width: 0,
        height: 0,
        area: area::init_trivial(),
        min: 0,
        max: 100,
        palette: palettes::init_trivial(),
        palette_zero: palettes::init_trivial(),
        resolution_multiplier: ResolutionMultiplier::Single,
    }
}

impl Machine {
    pub fn calculate<'lt, M: MemType<M>>(
        &self,
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
    fn chunk_calculation<'lt, M: MemType<M>>(&self, xy: &[u32; 2], fractal: &dyn FractalMath<M>) {
        let (x_from, x_to, y_from, y_to) = chunk_boundaries(xy, fractal.width(), fractal.height());
        for x in x_from..x_to {
            for y in y_from..y_to {
                self.calculate_path_xy(x, y, fractal);
            }
        }
    }

    fn chunk_calculation_with_wrap<'lt, M: MemType<M>>(
        &self,
        xy: &[u32; 2],
        fractal: &dyn FractalMath<M>,
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

    pub fn move_target(&self, x: usize, y: usize) {
        self.area.move_target(x, y);
    }

    pub fn zoom_in_recalculate_pixel_positions(&self, is_mandelbrot: bool) {
        self.area.zoom_in();
        window::paint_image_calculation_progress(&self.data_image);

        self.recalculate_pixels_positions_for_next_calculation(is_mandelbrot);
        window::paint_image_calculation_progress(&self.data_image);
    }

    pub fn zoom_in(&self) {
        self.area.zoom_in();
    }

    // This is called after calculation finished, a zoom-in was called and new area measures recalculated
    pub fn recalculate_pixels_positions_for_next_calculation(&self, is_mandelbrot: bool) {
        println!("recalculate_pixels_positions_for_next_calculation()");
        // Scan all elements : old positions from previous calculation
        // Some elements will be moved to new positions
        // For all the moved elements, subsequent calculations will be skipped.
        let area = &self.area;
        let (cx, cy) = area.point_to_pixel(
            area.data.lock().unwrap().center_re,
            area.data.lock().unwrap().center_im,
        );
        now("1. move top left to center");
        for y in 0..cy {
            for x in 0..cx {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        now("2. move top right to center");
        for y in 0..cy {
            for x in (cx..self.width).rev() {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        now("3. move bottom left to center");
        for y in (cy..self.height).rev() {
            for x in 0..cx {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        now("4. move bottom right to center");
        for y in (cy..self.height).rev() {
            for x in (cx..self.width).rev() {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        // Create new elements on positions where no px moved to
        now("fill empty places");
        let mut c_moved = 0;
        let mut c_created = 0;

        let res = area.screen_to_domain_re_copy();
        let ims = area.screen_to_domain_im_copy();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut mo_px = self.data_image.mo_px_at(x, y);
                if mo_px.is_none() {
                    c_created += 1;

                    let re = res[x];
                    let im = ims[y];

                    if self
                        .data_image
                        .all_neighbors_finished_bad(x, y, is_mandelbrot)
                    {
                        // Calculation for some positions should be skipped as they are too far away form any long successful divergent position
                        mo_px.replace(hibernated_deep_black(re, im));
                    } else {
                        mo_px.replace(active_new(re, im));
                    }
                } else {
                    c_moved += 1;
                }
            }
        }
        println!("moved:     {}", c_moved);
        println!("created:   {}", c_created);
        assert!(c_moved > 0);
        assert!(c_created > 0);
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

    // TODO why 400?

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
