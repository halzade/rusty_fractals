use crate::area::Area;
use crate::constants::CALCULATION_BOUNDARY;
use crate::data_image::DataImage;
use crate::data_image::DataType::Static;
use crate::data_px::{active_new, hibernated_deep_black};
use crate::fractal::CalculationType::StaticImage;
use crate::fractal::{
    CalculationType, FractalConfig, FractalMath, FractalType, MemType, OrbitType,
};
use crate::fractal_log::now;
use crate::palette::Palette;
use crate::palettes::new_palette_by_name;
use crate::perfect_colour_distribution::perfectly_colour_nebula_values;
use crate::pixel_states::DomainElementState;
use crate::pixel_states::DomainElementState::{FinishedSuccess, FinishedTooLong, FinishedTooShort};
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::{area, data_image, machine, palettes, pixel_states, window};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::thread;

/**
 * Machine owns all data
 */
pub struct Machine<'lt> {
    pub fractal_name: &'lt str,
    pub fractal_type: FractalType,
    // area config
    pub area: Area<'lt>,
    pub width_x: usize,  // width x in pixels
    pub height_y: usize, // with y in pixels
    pub width_re: f64,
    pub center_re: f64,
    pub center_im: f64,
    // display config
    pub data_image: DataImage<'lt>,
    pub width: i32,
    pub height: i32,
    pub palette: Palette<'lt>,
    // mandelbrot specific
    // used to color the (black) inside of Mandelbrot set
    pub palette_zero: Palette<'lt>,
    // calculation config
    pub calc_type: CalculationType,
    pub orbits: OrbitType, // fractal::finite_orbits / infinite_orbits
    pub iteration_min: u32,
    pub iteration_max: u32,
    pub update_max: u32,
    pub update_min: u32,

    //  nebula specific - use multiple numbers for each screen pixel
    pub resolution_multiplier: ResolutionMultiplier,
}

pub fn init<'lt>(name: &'lt str, config: &FractalConfig) -> Machine<'lt> {
    let area: Area<'_> = area::init(config);
    let wx = area.data.lock().unwrap().width_x as i32;
    let hy = area.data.lock().unwrap().height_y as i32;
    Machine {
        fractal_name: name,
        data_image: data_image::init(Static, &area), // TODO Dynamic
        width: wx,
        height: hy,
        area,
        width_x: config.width_x,
        height_y: config.height_y,
        width_re: config.width_re,
        center_re: config.center_re,
        center_im: config.center_im,
        // mandelbrot fractals calculate from 0
        // nebula fractals include only calculations longer then min
        iteration_min: config.iteration_min,
        iteration_max: config.iteration_max,
        palette: new_palette_by_name(&config.palette),
        palette_zero: new_palette_by_name(&config.palette_zero),
        calc_type: config.calc_type,
        resolution_multiplier: config.resolution_multiplier,
        fractal_type: config.fractal_type,
        orbits: OrbitType::Finite,
        update_max: config.update_max,
        update_min: config.update_min,
    }
}

pub fn init_trivial<'lt>() -> Machine<'lt> {
    Machine {
        fractal_name: "Trivial Fractal",
        data_image: data_image::init_trivial(),
        width: 2,
        height: 2,
        area: area::init_trivial(),
        width_x: 2,
        height_y: 2,
        width_re: 2.0,
        center_re: 0.0,
        center_im: 0.0,
        palette: palettes::init_trivial(),
        palette_zero: palettes::init_trivial(),
        calc_type: StaticImage,
        orbits: OrbitType::Finite,
        iteration_min: 0,
        iteration_max: 10,
        update_max: 3,
        update_min: 1,
        resolution_multiplier: ResolutionMultiplier::Single,
        fractal_type: FractalType::Mandelbrot,
    }
}

impl Machine {
    /**
     * Calculate the whole Nebula fractal
     */
    pub fn calculate<'lt, M: MemType<M>>(&self, fractal: &dyn FractalMath<M>) {
        // TODO
        thread::spawn(move || {
            let mut ma: Machine = machine::init();
            ma.calculate(fractal);
            // TODO or
            // TODO calculate_mandelbrot();
        });

        println!("calculate()");
        let coordinates_xy: Vec<[u32; 2]> = shuffled_calculation_coordinates();
        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation(&xy, fractal);
            // window refresh
            window::paint_image_calculation_progress(&self.data_image);
        });
        self.data_image.recalculate_pixels_states();

        if self.resolution_multiplier != ResolutionMultiplier::Single {
            println!("calculate() with wrap");
            // previous calculation completed, calculate more elements
            coordinates_xy.par_iter().for_each(|xy| {
                // calculation
                self.chunk_calculation_with_wrap(&xy);
                // window refresh
                // TODO window::paint_image_calculation_progress(&data);
                window::paint_path(&self.area, &self.data_image);
            });
        }
        perfectly_colour_nebula_values(&self.data_image, &self.palette);
        window::paint_image_result(&self.data_image());
    }

    // in sequence executes as 20x20 parallel for each image part/chunk
    fn chunk_calculation<'lt, M: MemType<M>>(&self, xy: &[u32; 2], fractal: &dyn FractalMath<M>) {
        let (x_from, x_to, y_from, y_to) = self.chunk_boundaries(xy);
        for x in x_from..x_to {
            for y in y_from..y_to {
                self.calculate_path_xy(x, y, fractal);
            }
        }
    }

    fn chunk_calculation_with_wrap<'lt, M: MemType<M>>(&self, xy: &[u32; 2]) {
        if self.resolution_multiplier == ResolutionMultiplier::Single {
            panic!()
        }
        let (x_from, x_to, y_from, y_to) = self.chunk_boundaries(xy);
        let plank = self.area.plank();
        for x in x_from..x_to {
            for y in y_from..y_to {
                if self.data_image.is_on_mandelbrot_horizon(x, y) {
                    let (origin_re, origin_im) = self.data_image.origin_at(x, y);
                    let wrap = self.data_image.wrap(
                        origin_re,
                        origin_im,
                        self.resolution_multiplier,
                        plank,
                    );
                    // within the same pixel
                    for [re, im] in wrap {
                        self.calculate_path(re, im, true);
                    }
                }
            }
        }
    }

    fn calculate_path_xy<'lt, M: MemType<M>>(
        &self,
        x: usize,
        y: usize,
        fractal: &dyn FractalMath<M>,
    ) {
        let (state, origin_re, origin_im) = self.data_image.state_origin_at(x, y);
        if pixel_states::is_active_new(state) {
            let (iterator, path_length) = self.calculate_path(fractal, origin_re, origin_im, false);
            let state = self.state_from_path_length(iterator, path_length);
            self.data_image.set_pixel_state(x, y, state);
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

    pub fn chunk_boundaries(&self, xy: &[u32; 2]) -> (usize, usize, usize, usize) {
        let chunk_size_x = (self.width / 20) as u32;
        let chunk_size_y = (self.height / 20) as u32;
        (
            (xy[0] * chunk_size_x) as usize,
            ((xy[0] + 1) * chunk_size_x) as usize,
            (xy[1] * chunk_size_y) as usize,
            ((xy[1] + 1) * chunk_size_y) as usize,
        )
    }

    pub fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        if &self.calculation_config.orbits == OrbitType::Finite {
            // only the edges of mandelbrot set
            length > min && iterator < max
        } else {
            // also contains the inside of mandelbrot set
            length > min && iterator == max
        }
    }

    pub fn calculate_path<'lt, T: MemType<T>>(
        fractal: &impl FractalMath<T>,
        area: &Area,
        iteration_min: u32,
        iteration_max: u32,
        origin_re: f64,
        origin_im: f64,
        data_image: &DataImage,
        is_wrap: bool,
    ) -> (u32, u32) {
        let cb = CALCULATION_BOUNDARY as f64;
        let mut m: T = T::new(origin_re, origin_im);
        let mut iterator = 0;
        let mut length = 0;
        while m.quad() < cb && iterator < iteration_max {
            // Investigate if this is a good calculation path
            // Don't create path data yet. Too many origins don't produce good data
            // Most of the long and expensive calculations end up inside Mandelbrot set, useless
            // It is 1.68x faster to calculate path twice, and to record exclusively the good paths
            fractal.math(&mut m, origin_re, origin_im);
            if area.contains(m.re(), m.im()) {
                // this becomes important for zoom, when only a small amount
                // of calculation path elements is contained withing tiny area
                length += 1;
            }
            iterator += 1;
        }

        if path_test(calc_config, iteration_min, iteration_max, length, iterator) {
            // This origin produced good data
            // Record the calculation path
            let mut m: T = T::new(origin_re, origin_im);
            let mut path: Vec<[f64; 2]> = Vec::new();
            for _ in 0..iterator {
                fractal.math(&mut m, origin_re, origin_im);
                if area.contains(m.re(), m.im()) {
                    path.push([m.re(), m.im()]);
                }
            }

            // if iteration_max increased, ignore possible extension of previous calculation paths
            // path elements are going to migrate out of the screen shortly
            // removed last_iteration, last_visited_re, last_visited_im
            if data_image.is_dynamic() {
                data_image.save_path(path, is_wrap);
            } else {
                data_image.translate_path_to_point_grid(path, area, is_wrap);
            }
            // TODO stats.paths_new_points_amount += path.size();
        }
        (iterator, length)
    }

    pub fn state_from_path_length(&self, iterator: u32, path_length: u32) -> DomainElementState {
        if path_length < self.update_min {
            return FinishedTooShort;
        }
        if iterator == self.update_max {
            return FinishedTooLong;
        }
        FinishedSuccess
    }

    /* ------------------------------------------
     * Methods for Mandelbrot fractal calculation
     * ------------------------------------------
     */

    /*
     * Whole Mandelbrot calculation
     */
    pub fn calculate_mandelbrot<'lt, M: MemType<M>>(&self, fractal: &dyn FractalMath<M>) {
        println!("calculate_mandelbrot()");
        let coordinates_xy: Vec<[u32; 2]> = machine::shuffled_calculation_coordinates();


        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation_mandelbrot(fractal, xy);
            // window refresh
            window::paint_image_calculation_progress(fractal.data_image());
        });
        self.data_image.recalculate_pixels_states();
        perfectly_colour_mandelbrot_values(&self.data_image, &self.data_image.palette, &self.data_image.palette_zero);
        window::paint_image_result(&self.data_image);
    }

    fn chunk_calculation_mandelbrot<'lt, M: MemType<M>>(
        &self,
        fractal: &dyn FractalMath<M>,
        xy: &[u32; 2],
    ) {
        let (x_from, x_to, y_from, y_to) =
            self.chunk_boundaries(xy);
        for x in x_from..x_to {
            for y in y_from..y_to {
                let (state, origin_re, origin_im) = self.data_image.state_origin_at(x, y);
                // TODO, calculate only ActiveNew elements, copy quad and quid
                if !pixel_states::is_finished_any(state) {
                    // calculation
                    let (iterator, quad) =
                        self.calculate_path(fractal.max(), origin_re, origin_im);
                    let state = self. state_from_path_length(iterator, iterator);
                    self.data_image.set_pixel_mandelbrot(x, y, iterator, quad, state, self.iteration_max);
                }
            }
        }
    }

    pub fn calculate_mandelbrot_path<T: MemType<T>>(
        &self,
        fractal_math: &impl FractalMath<T>,
        iteration_max: u32,
        origin_re: f64,
        origin_im: f64,
    ) -> (u32, f64) {
        let cb = CALCULATION_BOUNDARY as f64;
        let mut m: T = T::new(origin_re, origin_im);
        let mut iterator = 0;
        while m.quad() < cb && iterator < iteration_max {
            fractal_math.math(&mut m, origin_re, origin_im);
            iterator += 1;
        }
        (iterator, m.quad())
    }
}

/**
 * Creates x,y pairs for calculation.
 * Then shuffles them, it looks better when rendering
 */
pub fn shuffled_calculation_coordinates() -> Vec<[u32; 2]> {
    let mut coordinates_xy: Vec<[u32; 2]> = Vec::new();

    // 400 little subdivisions of the screen
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
    use crate::machine;
    use crate::{area, data_image, fractal, machine};

    #[test]
    fn test_calculate_path_xy() {
        let fractal = fractal::init_trivial();
        let machine = machine.init_trivial();
        machine.calculate_path_xy(0, 0, 1, 5, &fractal);

        // TODO
    }

    #[test]
    fn test_chunk_calculation_mandelbrot<'lt>() {
        let mm = machine::init_trivial();

        let x = 0;
        let y = 0;
        let xy = [x, y];
        let fractal = fractal::init_trivial();
        mm.chunk_calculation_mandelbrot(&fractal, &xy);

        // TODO
    }

    #[test]
    fn test_calculate_path() {
        // prepare test data
        let area = area::init_trivial();
        let data_image = data_image::init_trivial();
        let fractal = fractal::init_trivial();

        // execute test
        let (iterator, length) =
            calculate_path(&fractal, &area, 1, 5, 0.0, 0.0, &data_image, false);

        assert_eq!(iterator, 5);
        assert_eq!(length, 0);
    }
}
